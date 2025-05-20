import csv
import json
from datetime import datetime, timedelta
from hashlib import sha256
from pathlib import Path
from typing import Any, Dict, List, Set, Tuple

import numpy as np
import pandas as pd
from constants import (
    BASE_DIR,
    CATEGORY_BALANCE_WEIGHT,
    COMPLETION_THRESHOLD,
    DAYS_FOR_TRACK_ROTATION,
    MAX_DAILY_TRACKS,
    PROGRESS_WEIGHT,
    RECENCY_WEIGHT,
    ROTATION_WEIGHT,
    TRACK_LOG_FILE,
    TRACK_STATS_FILE,
    UTC_PLUS_ONE,
)


def load_language_metadata(track_stats_file: Path) -> List[Dict[str, Any]]:
    """Load language metadata from JSON file."""
    try:
        with open(track_stats_file, "r") as f:
            return json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"Error loading metadata: {e}")
        return []


def count_completed_exercises(base_url: Path, track: str) -> int:
    """Count completed exercises for a given track (non-hidden subdirs)."""
    track_path = base_url / track
    if not track_path.exists() or not track_path.is_dir():
        return 0
    try:
        return sum(
            1 for d in track_path.iterdir() if d.is_dir() and not d.name.startswith(".")
        )
    except (PermissionError, FileNotFoundError) as e:
        print(f"Error accessing {track_path}: {e}")
        return 0


def load_track_selection_history(
    track_log_file: Path, days: int = DAYS_FOR_TRACK_ROTATION
) -> Dict[str, List[str]]:
    """Load track selection history for the past N days."""
    if not track_log_file.exists():
        return {}

    history = {}
    today = datetime.now(UTC_PLUS_ONE).date()
    cutoff_date = today - timedelta(days=days)

    try:
        with open(track_log_file, "r") as f:
            reader = csv.reader(f)
            next(reader, None)  # Skip header
            for row in reader:
                if len(row) >= 2:
                    date_str, tracks_str = row[0], row[1]
                    try:
                        date = datetime.strptime(date_str, "%Y-%m-%d").date()
                        if date >= cutoff_date:
                            tracks = tracks_str.split(",")
                            history[date_str] = tracks
                    except ValueError:
                        # Skip malformed dates
                        continue
    except Exception as e:
        print(f"Error reading track history: {e}")

    return history


def calculate_track_frequency(history: Dict[str, List[str]], track: str) -> float:
    """Calculate how frequently a track has been selected recently."""
    if not history:
        return 0.0

    occurrences = sum(1 for tracks in history.values() if track in tracks)
    return occurrences / len(history) if history else 0.0


def calculate_completion_factor(completed: int, total: int) -> float:
    """
    Calculate a completion factor that favors tracks in the middle of completion.
    Uses a bell curve that peaks around 50% completion.
    """
    if total == 0:
        return 0.0

    completion_ratio = completed / total

    # Bell curve factor - highest priority for tracks at ~50% completion
    # Drops for both new tracks (0%) and nearly completed tracks (100%)
    return np.sin(np.pi * completion_ratio)


def calculate_recency_factor(last_touched: int, max_last_touched: int) -> float:
    """
    Calculate recency factor. Tracks not touched recently get higher priority.
    Returns a value between 0 and 1, where 1 means highest priority.
    """
    if max_last_touched == 0:
        return 1.0

    # Normalize to 0-1 range and invert (0 days = recently touched = lower priority)
    return 1.0 - (last_touched / max_last_touched)


def calculate_rotation_factor(track_frequency: float) -> float:
    """
    Calculate rotation factor. Tracks selected less frequently get higher priority.
    Returns a value between 0 and 1, where 1 means highest priority.
    """
    # Invert frequency (less frequently selected = higher priority)
    return 1.0 - track_frequency


def compute_score(
    row: pd.Series,
    track_frequencies: Dict[str, float],
    category_counts: Dict[str, int],
    max_last_touched: int,
) -> float:
    """
    Advanced scoring function to prioritize tracks based on multiple factors.
    Higher score means higher priority for selection.
    """
    # Skip completed tracks
    if row["completion_ratio"] >= COMPLETION_THRESHOLD:
        return -1.0  # Lowest possible score

    # Calculate component scores
    progress_score = calculate_completion_factor(row["completed"], row["total"])
    recency_score = calculate_recency_factor(row["last_touched"], max_last_touched)

    # Calculate rotation factor based on selection history
    track_frequency = track_frequencies.get(row["language"], 0.0)
    rotation_score = calculate_rotation_factor(track_frequency)

    # Category balance - favor underrepresented categories
    total_categories = sum(category_counts.values())
    category_ratio = (
        category_counts.get(row["category"], 0) / total_categories
        if total_categories > 0
        else 0
    )
    category_balance_score = 1.0 - category_ratio  # Lower ratio = higher score

    # Combine all factors with weights
    final_score = (
        progress_score * PROGRESS_WEIGHT
        + recency_score * RECENCY_WEIGHT
        + rotation_score * ROTATION_WEIGHT
        + category_balance_score * CATEGORY_BALANCE_WEIGHT
    )

    return final_score


def get_daily_seed() -> Tuple[int, str]:
    """Generate a deterministic seed based on today's date."""
    today_str = datetime.now(UTC_PLUS_ONE).strftime("%Y-%m-%d")
    seed = int(sha256(today_str.encode()).hexdigest(), 16) % (10**8)
    return seed, today_str


def select_daily_tracks(
    df: pd.DataFrame, seed: int, max_tracks: int = MAX_DAILY_TRACKS
) -> List[str]:
    """
    Select top tracks for the day using a two-phase approach:
    1. Try to select one track from each category first
    2. Fill remaining slots with highest scoring tracks
    """
    # Randomize the order slightly using the seed but keep the score ordering
    df_sorted = df.sample(frac=1, random_state=seed).sort_values(
        by="score", ascending=False
    )

    # Get unique categories
    categories = df_sorted["category"].unique()

    selected: List[str] = []
    seen_categories: Set[str] = set()

    # Phase 1: Try to select one track from each category first
    for category in categories:
        if len(selected) >= max_tracks:
            break

        # Get highest scoring track for this category that isn't completed
        category_tracks = df_sorted[
            (df_sorted["category"] == category)
            & (df_sorted["score"] > 0)  # Skip completed tracks
        ]

        if not category_tracks.empty:
            best_track = category_tracks.iloc[0]["language"]
            selected.append(best_track)
            seen_categories.add(category)

    # Phase 2: Fill remaining slots with highest scoring tracks
    remaining_slots = max_tracks - len(selected)
    if remaining_slots > 0:
        # Filter out already selected tracks and completed tracks
        remaining_tracks = df_sorted[
            (~df_sorted["language"].isin(selected))
            & (df_sorted["score"] > 0)  # Skip completed tracks
        ]

        # Add highest scoring remaining tracks
        for _, row in remaining_tracks.iterrows():
            if len(selected) >= max_tracks:
                break
            selected.append(row["language"])

    return selected


def ensure_log_file_exists(track_log_file: Path) -> None:
    """Ensure the log file exists with proper header."""
    if not track_log_file.exists():
        track_log_file.parent.mkdir(parents=True, exist_ok=True)
        track_log_file.write_text("date,tracks\n")


def is_already_logged(track_log_file: Path, today_str: str) -> bool:
    """Check if today's selection is already logged."""
    if not track_log_file.exists():
        return False

    try:
        existing_lines = track_log_file.read_text().splitlines()
        existing_dates = {
            line.split(",")[0] for line in existing_lines[1:] if "," in line
        }
        return today_str in existing_dates
    except Exception as e:
        print(f"Error checking log file: {e}")
        return False


def log_daily_selection(
    track_log_file: Path, today_str: str, selected_tracks: List[str]
) -> None:
    """Log today's track selection to CSV file."""
    try:
        with open(track_log_file, "a", newline="") as f:
            writer = csv.writer(f)
            writer.writerow([today_str, ",".join(selected_tracks)])
    except Exception as e:
        print(f"Error logging selection: {e}")


def main() -> None:
    """Main function to orchestrate the daily track selection."""
    # Load and prepare data
    languages = load_language_metadata(TRACK_STATS_FILE)
    if not languages:
        print("No language metadata found. Exiting.")
        return

    df = pd.DataFrame(languages)

    # Calculate completion stats
    df["completed"] = df["language"].apply(
        lambda track: count_completed_exercises(BASE_DIR, track)
    )
    df["completion_ratio"] = df["completed"] / df["total"]

    # Load track selection history
    history = load_track_selection_history(TRACK_LOG_FILE)

    # Calculate track frequencies from history
    track_frequencies = {
        track: calculate_track_frequency(history, track) for track in df["language"]
    }

    # Calculate category distribution
    category_counts = df["category"].value_counts().to_dict()

    # Find maximum last_touched value for normalization
    max_last_touched = df["last_touched"].max()

    # Calculate scores for prioritization
    df["score"] = df.apply(
        lambda row: compute_score(
            row, track_frequencies, category_counts, max_last_touched
        ),
        axis=1,
    )

    # Get deterministic daily selection
    seed, today_str = get_daily_seed()
    selected_tracks = select_daily_tracks(df, seed)

    # Log the selection (if not already logged)
    ensure_log_file_exists(TRACK_LOG_FILE)
    if not is_already_logged(TRACK_LOG_FILE, today_str):
        log_daily_selection(TRACK_LOG_FILE, today_str, selected_tracks)

    # Output the selection
    print(f"Selected tracks for {today_str}:")
    for track in selected_tracks:
        track_data = df[df["language"] == track].iloc[0]
        print(
            f"- {track} ({track_data['category']}): "
            f"{track_data['completed']}/{track_data['total']} exercises completed "
            f"({track_data['completion_ratio']:.0%})"
        )


if __name__ == "__main__":
    main()
