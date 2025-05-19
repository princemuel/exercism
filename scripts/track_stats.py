import json
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional


def load_track_stats(file_path: Path) -> List[Dict]:
    """Load track statistics from JSON file."""
    if not file_path.exists():
        raise FileNotFoundError(f"Missing: {file_path}")

    with open(file_path, "r") as f:
        return json.load(f)


def save_track_stats(file_path: Path, stats: List[Dict]) -> None:
    """Save track statistics to JSON file."""
    with open(file_path, "w") as f:
        json.dump(stats, f, indent=4)


def is_hidden_path(file_path: Path) -> bool:
    """Check if any part of the path starts with a dot (hidden)."""
    return any(part.startswith(".") for part in file_path.parts)


def get_file_modification_time(file_path: Path) -> Optional[float]:
    """Get file modification time, handling errors gracefully."""
    try:
        return file_path.stat().st_mtime
    except (OSError, PermissionError):
        return None


def find_most_recent_file(track_dir: Path) -> Optional[datetime]:
    """Find the most recently modified file in a track directory."""
    if not track_dir.exists() or not track_dir.is_dir():
        return None

    most_recent_time = 0.0

    # Recursively find all files, ignoring hidden directories
    for file_path in track_dir.rglob("*"):
        if file_path.is_file() and not is_hidden_path(file_path):
            mtime = get_file_modification_time(file_path)
            if mtime and mtime > most_recent_time:
                most_recent_time = mtime

    return datetime.fromtimestamp(most_recent_time) if most_recent_time > 0 else None


def calculate_days_since(date: datetime) -> int:
    """Calculate days between a date and now."""
    return (datetime.now() - date).days


def get_track_directory(base_dir: Path, track_name: str) -> Path:
    """Get the directory path for a given track."""
    return base_dir / track_name


def update_track_last_touched(entry: Dict, base_dir: Path) -> Optional[int]:
    """Update last_touched for a single track entry. Returns new value if changed."""
    track_name = entry["language"]
    track_dir = get_track_directory(base_dir, track_name)

    # Find most recent file in this track
    most_recent_date = find_most_recent_file(track_dir)

    if not most_recent_date:
        return None

    days_since = calculate_days_since(most_recent_date)

    # Return new value if different, None if unchanged
    if entry["last_touched"] != days_since:
        old_value = entry["last_touched"]
        entry["last_touched"] = days_since
        return old_value

    return None


def print_track_update(
    track_name: str, old_value: Optional[int], new_value: int
) -> None:
    """Print update status for a track."""
    if old_value is not None:
        print(f"✅ {track_name}: {old_value} → {new_value} days")
    else:
        print(f"✓ {track_name}: {new_value} days (unchanged)")


def print_track_no_files(track_name: str, track_dir: Path) -> None:
    """Print warning when no files found in track."""
    print(f"⚠️  {track_name}: No files found in {track_dir}")


def process_all_tracks(stats: List[Dict], base_dir: Path) -> int:
    """Process all tracks and return count of updated entries."""
    updated_count = 0

    print("🔍 Scanning track directories for recent files...")

    for entry in stats:
        track_name = entry["language"]
        old_value = update_track_last_touched(entry, base_dir)

        if old_value is not None:
            # Track was updated
            updated_count += 1
            print_track_update(track_name, old_value, entry["last_touched"])
        elif entry.get("last_touched") is not None:
            # Track unchanged but has value
            print_track_update(track_name, None, entry["last_touched"])
        else:
            # No files found
            track_dir = get_track_directory(base_dir, track_name)
            print_track_no_files(track_name, track_dir)

    return updated_count


def print_summary(updated_count: int, file_name: str) -> None:
    """Print final summary of changes."""
    if updated_count > 0:
        print(f"\n🎯 Updated {updated_count} entries in {file_name}")
    else:
        print("\n✅ No changes needed. All entries are up to date.")


def main() -> None:
    """Main function to orchestrate the last_touched update process."""

    # === CONFIG ===
    BASE_DIR = Path.home() / "exercism"
    TRACK_STATS_FILE = BASE_DIR / "database" / "track_stats.json"

    try:
        # Load existing stats
        stats = load_track_stats(TRACK_STATS_FILE)

        # Process all tracks
        updated_count = process_all_tracks(stats, BASE_DIR)

        # Save only if something changed
        if updated_count > 0:
            save_track_stats(TRACK_STATS_FILE, stats)

        # Print summary
        print_summary(updated_count, TRACK_STATS_FILE.name)

    except FileNotFoundError as e:
        print(f"❌ {e}")
    except Exception as e:
        print(f"❌ Unexpected error: {e}")


if __name__ == "__main__":
    main()
