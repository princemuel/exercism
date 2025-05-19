from datetime import timedelta, timezone
from pathlib import Path

# Timezone
UTC_PLUS_ONE = timezone(timedelta(hours=1))

# Setup paths
BASE_DIR = Path.home() / "exercism"
TRACK_STATS_FILE = BASE_DIR / "database" / "track_stats.json"
TRACK_LOG_FILE = BASE_DIR / "database" / "track_log.csv"
