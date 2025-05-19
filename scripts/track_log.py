import csv
import json
from datetime import datetime, timezone
from hashlib import sha256
from pathlib import Path
from typing import Any, Dict, List, Tuple

import pandas as pd


def load_language_metadata(track_stats_file: Path) -> List[Dict[str, Any]]:
    """Load language metadata from JSON file."""
    with open(track_stats_file, "r") as f:
        return json.load(f)


def count_completed_exercises(base_url: Path, track: str) -> int:
    """Count completed exercises for a given track (non-hidden subdirs)."""
    track_path = base_url / track
    if not track_path.exists() or not track_path.is_dir():
        return 0
    return sum(
        1 for d in track_path.iterdir() if d.is_dir() and not d.name.startswith(".")
    )


def compute_score(row: pd.Series) -> float:
    """Scoring function to prioritize tracks based on multiple factors."""
    progress_factor = row["remaining"] / row["total"] if row["total"] > 0 else 1.0
    recency_factor = row["last_touched"] / 100.0  # normalized
    category_weight = 1.0  # can be tuned later if needed
    return progress_factor + recency_factor + category_weight


def get_daily_seed() -> Tuple[int, str]:
    """Generate a deterministic seed based on today's date."""
    today_str = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    seed = int(sha256(today_str.encode()).hexdigest(), 16) % (10**8)
    return seed, today_str


def select_daily_tracks(df: pd.DataFrame, seed: int, max_tracks: int = 5) -> List[str]:
    """Select top tracks for the day, ensuring unique categories if possible."""
    df_sorted = df.sample(frac=1, random_state=seed).sort_values(
        by="score", ascending=False
    )

    selected: List[str] = []
    seen_categories: set[str] = set()

    for _, row in df_sorted.iterrows():
        if row["category"] not in seen_categories or len(seen_categories) >= max_tracks:
            selected.append(row["language"])
            seen_categories.add(row["category"])
            if len(selected) == max_tracks:
                break

    return selected


def ensure_log_file_exists(track_log_file: Path) -> None:
    """Ensure the log file exists with proper header."""
    if not track_log_file.exists():
        track_log_file.write_text("date,tracks\n")


def is_already_logged(track_log_file: Path, today_str: str) -> bool:
    """Check if today's selection is already logged."""
    if not track_log_file.exists():
        return False

    existing_lines = track_log_file.read_text().splitlines()
    existing_dates = {line.split(",")[0] for line in existing_lines[1:]}
    return today_str in existing_dates


def log_daily_selection(
    track_log_file: Path, today_str: str, selected_tracks: List[str]
) -> None:
    """Log today's track selection to CSV file."""
    with open(track_log_file, "a", newline="") as f:
        writer = csv.writer(f)
        writer.writerow([today_str, ",".join(selected_tracks)])


def main() -> None:
    """Main function to orchestrate the daily track selection."""
    # Setup paths
    BASE_DIR = Path.home() / "exercism"
    TRACK_STATS_FILE = BASE_DIR / "database" / "track_stats.json"
    TRACK_LOG_FILE = BASE_DIR / "database" / "track_log.csv"

    # Load and prepare data
    languages = load_language_metadata(TRACK_STATS_FILE)
    df = pd.DataFrame(languages)

    # Calculate completion stats
    df["completed"] = df["language"].apply(
        lambda track: count_completed_exercises(BASE_DIR, track)
    )
    df["remaining"] = df["total"] - df["completed"]

    # Calculate scores for prioritization
    df["score"] = df.apply(compute_score, axis=1)

    # Get deterministic daily selection
    seed, today_str = get_daily_seed()
    selected_tracks = select_daily_tracks(df, seed)

    # Log the selection (if not already logged)
    ensure_log_file_exists(TRACK_LOG_FILE)
    if not is_already_logged(TRACK_LOG_FILE, today_str):
        log_daily_selection(TRACK_LOG_FILE, today_str, selected_tracks)

    # Output the selection
    print(selected_tracks)


if __name__ == "__main__":
    main()
