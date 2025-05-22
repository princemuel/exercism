import csv
import json
from datetime import datetime, timedelta
from decimal import Decimal, getcontext
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

# Set decimal precision for consistent floating-point operations
getcontext().prec = 28


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


def calculate_completion_factor(completed: int, total: int) -> Decimal:
    """
    Calculate a completion factor that favors tracks in the middle of completion.
    Uses a bell curve that peaks around 50% completion.
    Returns Decimal for precision.
    """
    if total == 0:
        return Decimal("0.0")

    completion_ratio = Decimal(completed) / Decimal(total)

    # Convert to float for sin calculation, then back to Decimal
    # This maintains more precision than using numpy throughout
    sin_value = np.sin(float(completion_ratio) * np.pi)
    return Decimal(str(sin_value))


def calculate_recency_factor(last_touched: int, max_last_touched: int) -> Decimal:
    """
    Calculate recency factor. Tracks not touched recently get higher priority.
    Returns a Decimal between 0 and 1, where 1 means highest priority.
    """
    if max_last_touched == 0:
        return Decimal("1.0")

    # Normalize to 0-1 range and invert
    return Decimal("1.0") - (Decimal(int(last_touched)) / Decimal(int(max_last_touched)))


def calculate_rotation_factor(track_frequency: float) -> Decimal:
    """
    Calculate rotation factor. Tracks selected less frequently get higher priority.
    Returns a Decimal between 0 and 1, where 1 means highest priority.
    """
    return Decimal("1.0") - Decimal(str(track_frequency))


def compute_score(
    row: pd.Series,
    track_frequencies: Dict[str, float],
    category_counts: Dict[str, int],
    max_last_touched: int,
) -> float:
    """
    Advanced scoring function to prioritize tracks based on multiple factors.
    Higher score means higher priority for selection.
    Returns Decimal for consistent precision.
    """
    # Skip completed tracks
    if row["completion_ratio"] >= COMPLETION_THRESHOLD:
        return -1.0  # Lowest possible score

    # Calculate component scores using Decimals
    progress_score = calculate_completion_factor(row["completed"], row["total"])
    recency_score = calculate_recency_factor(row["last_touched"], max_last_touched)

    # Calculate rotation factor based on selection history
    track_frequency = track_frequencies.get(row["language"], 0.0)
    rotation_score = calculate_rotation_factor(track_frequency)

    # Category balance - favor underrepresented categories
    total_categories = sum(category_counts.values())
    category_ratio = (
        Decimal(category_counts.get(row["category"], 0)) / Decimal(total_categories)
        if total_categories > 0
        else Decimal("0")
    )
    category_balance_score = Decimal("1.0") - category_ratio

    # Combine all factors with weights (convert weights to Decimals)
    final_score = (
        progress_score * Decimal(str(PROGRESS_WEIGHT))
        + recency_score * Decimal(str(RECENCY_WEIGHT))
        + rotation_score * Decimal(str(ROTATION_WEIGHT))
        + category_balance_score * Decimal(str(CATEGORY_BALANCE_WEIGHT))
    )

    return float(final_score)


def create_deterministic_tie_breaker(seed: int, languages: List[str]) -> Dict[str, int]:
    """
    Create a deterministic tie-breaker for languages based on the daily seed.
    Each language gets a unique rank that's consistent for the day.
    """
    # Create a deterministic ordering based on seed and language name
    np.random.seed(seed)

    # Create pairs of (random_value, language) and sort by random_value
    random_pairs = [(np.random.random(), lang) for lang in sorted(languages)]
    random_pairs.sort()

    # Create rank mapping (lower rank = higher priority in ties)
    return {lang: rank for rank, (_, lang) in enumerate(random_pairs)}


def select_daily_tracks(
    df: pd.DataFrame, seed: int, max_tracks: int = MAX_DAILY_TRACKS
) -> List[str]:
    """
    Select top tracks for the day using a deterministic approach with proper tie-breaking.
    """
    # Filter out completed tracks first
    df_valid = df[df["score"] > Decimal("0")].copy()

    if df_valid.empty:
        return []

    # Create deterministic tie-breaker ranks
    tie_breaker_ranks = create_deterministic_tie_breaker(
        seed, df_valid["language"].tolist()
    )
    df_valid["tie_breaker"] = df_valid["language"].map(tie_breaker_ranks)

    # Sort by score (descending), then by tie_breaker rank (ascending), then by language name
    # This ensures complete determinism even with identical scores
    df_sorted = df_valid.sort_values(
        by=["score", "tie_breaker", "language"], ascending=[False, True, True]
    )

    # Get unique categories in deterministic order
    categories = sorted(df_sorted["category"].unique())

    selected: List[str] = []
    seen_categories: Set[str] = set()

    # Phase 1: Try to select one track from each category first
    for category in categories:
        if len(selected) >= max_tracks:
            break

        # Get tracks for this category, maintaining sort order
        category_tracks = df_sorted[df_sorted["category"] == category]

        if not category_tracks.empty:
            # Select the highest scoring track for this category (first in sorted order)
            best_track = category_tracks.iloc[0]["language"]
            selected.append(best_track)
            seen_categories.add(category)

    # Phase 2: Fill remaining slots with highest scoring tracks
    remaining_slots = max_tracks - len(selected)
    if remaining_slots > 0:
        # Filter out already selected tracks, maintaining sort order
        remaining_tracks = df_sorted[~df_sorted["language"].isin(selected)]

        # Add highest scoring remaining tracks
        for _, row in remaining_tracks.head(remaining_slots).iterrows():
            selected.append(row["language"])

    return selected


def get_daily_seed() -> Tuple[int, str]:
    """Generate a deterministic seed based on today's date."""
    today_str = datetime.now(UTC_PLUS_ONE).strftime("%Y-%m-%d")
    seed = int(sha256(today_str.encode()).hexdigest(), 16) % (10**8)
    return seed, today_str


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


def get_cached_completion_counts(today_str: str) -> Dict[str, int]:
    """Get cached completion counts for today, or generate new ones if not cached."""
    cache_file = BASE_DIR / "database" / f"completion_cache_{today_str}.json"

    if cache_file.exists():
        try:
            with open(cache_file, "r") as f:
                return json.load(f)
        except (json.JSONDecodeError, FileNotFoundError):
            pass  # Fall through to regenerate cache

    # Cache doesn't exist or is invalid, generate new counts
    languages = load_language_metadata(TRACK_STATS_FILE)
    completion_counts = {}

    for lang_data in languages:
        track = lang_data["language"]
        completed = count_completed_exercises(BASE_DIR, track)
        completion_counts[track] = completed

    # Save to cache
    cache_file.parent.mkdir(parents=True, exist_ok=True)
    with open(cache_file, "w") as f:
        json.dump(completion_counts, f, indent=2)

    return completion_counts


def main() -> None:
    """Main function to orchestrate the daily track selection."""
    # Get deterministic daily selection seed first
    seed, today_str = get_daily_seed()

    # Load and prepare data
    languages = load_language_metadata(TRACK_STATS_FILE)
    if not languages:
        print("No language metadata found. Exiting.")
        return

    df = pd.DataFrame(languages)

    # Get cached completion counts (deterministic for the day)
    completion_counts = get_cached_completion_counts(today_str)
    df["completed"] = df["language"].map(completion_counts).fillna(0).astype(int)
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

    # Calculate scores for prioritization using Decimals
    df["score"] = df.apply(
        lambda row: compute_score(
            row, track_frequencies, category_counts, max_last_touched
        ),
        axis=1,
    )

    # Get deterministic daily selection
    selected_tracks = select_daily_tracks(df, seed)

    # Log the selection (if not already logged)
    ensure_log_file_exists(TRACK_LOG_FILE)
    if not is_already_logged(TRACK_LOG_FILE, today_str):
        log_daily_selection(TRACK_LOG_FILE, today_str, selected_tracks)

    # Output the selection with score information for debugging
    print(f"Selected tracks for {today_str} (seed: {seed}):")
    for track in selected_tracks:
        track_data = df[df["language"] == track].iloc[0]
        print(
            f"- {track} ({track_data['category']}): "
            f"{track_data['completed']}/{track_data['total']} exercises completed "
            f"({track_data['completion_ratio']:.0%}), score: {track_data['score']:.6f}"
        )


if __name__ == "__main__":
    main()
