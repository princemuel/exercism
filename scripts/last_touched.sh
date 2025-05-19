#!/usr/bin/env bash

BASE_DIR="$(dirname "$(dirname "$(realpath "$0")")")"
TRACKS_FILE="$BASE_DIR/database/tracks.txt"
EXERCISM_DIR="$BASE_DIR"
LAST_TOUCHED_FILE="$BASE_DIR/database/last_touched.json"

# Validate tracks file exists
if [ ! -f "$TRACKS_FILE" ]; then
  echo "❌ Error: tracks.txt not found at $TRACKS_FILE"
  exit 1
fi

# Init touched file if needed
touch "$LAST_TOUCHED_FILE"
[ ! -s "$LAST_TOUCHED_FILE" ] && echo "{}" > "$LAST_TOUCHED_FILE"

echo "🔍 Scanning exercise folders..."

while read -r lang; do
  dir="$EXERCISM_DIR/$lang"
  if [ -d "$dir" ]; then
    latest=$(find "$dir" -type f -printf '%T+ %p\n' 2>/dev/null | sort -r | head -n 1 | cut -d' ' -f1)
    if [ -n "$latest" ]; then
      jq --arg lang "$lang" --arg date "${latest:0:10}" '.[$lang] = $date' "$LAST_TOUCHED_FILE" > tmp.$$.json && mv tmp.$$.json "$LAST_TOUCHED_FILE"
      echo "✅ $lang updated: ${latest:0:10}"
    else
      echo "⚠️ $lang has no exercise files yet."
    fi
  else
    echo "🚫 Skipping $lang — directory not found."
  fi
done < "$TRACKS_FILE"

echo "📁 Last touched info saved to: $LAST_TOUCHED_FILE"
