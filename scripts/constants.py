from datetime import timedelta, timezone
from pathlib import Path

# Timezone
UTC_PLUS_ONE = timezone(timedelta(hours=1))

# Setup paths
BASE_DIR = Path.home() / "exercism"
TRACK_STATS_FILE = BASE_DIR / "database" / "track_stats.json"
TRACK_LOG_FILE = BASE_DIR / "database" / "track_log.csv"

# Constants for algorithm tuning
MAX_DAILY_TRACKS = 5
DAYS_FOR_TRACK_ROTATION = 14  # Days to consider for track rotation history
COMPLETION_THRESHOLD = 1.0  # Consider a track completed at 100% completion

# Weights for score components (using exact decimal representations)
PROGRESS_WEIGHT = 0.3
RECENCY_WEIGHT = 0.2
ROTATION_WEIGHT = 0.3
CATEGORY_BALANCE_WEIGHT = 0.2

# Determinism settings
DECIMAL_PRECISION = 28  # Precision for Decimal calculations
RANDOM_SEED_MOD = 2**256  # Modulo for daily seed generation

# Validation: Ensure weights sum to 1.0
assert (
    abs(
        PROGRESS_WEIGHT
        + RECENCY_WEIGHT
        + ROTATION_WEIGHT
        + CATEGORY_BALANCE_WEIGHT
        - 1.0
    )
    < 1e-10
), "Score weights must sum to 1.0"
