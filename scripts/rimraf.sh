#!/usr/bin/env bash
set -euo pipefail

# Configuration
DIRS_TO_DELETE=("target" ".pytest_cache" "__pycache__" "node_modules" ".yarn" ".venv")
FILES_TO_DELETE=(".editorconfig")

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Function to safely delete items
safe_delete() {
    local item="$1"
    local type="$2"
    if [[ -e "$item" ]]; then
        echo -e "${YELLOW}Deleting $type:${NC} $item"
        rm -rf "$item"
        echo -e "${GREEN}✓ Deleted${NC}"
    fi
}

# Build find expression for directories
dir_expr=""
for dir in "${DIRS_TO_DELETE[@]}"; do
    if [[ -n "$dir_expr" ]]; then
        dir_expr="$dir_expr -o"
    fi
    dir_expr="$dir_expr -name $dir"
done

# Build find expression for files
file_expr=""
for file in "${FILES_TO_DELETE[@]}"; do
    if [[ -n "$file_expr" ]]; then
        file_expr="$file_expr -o"
    fi
    file_expr="$file_expr -name $file"
done

echo -e "${GREEN}Starting cleanup...${NC}"

# Remove directories (exclude base directory but include all subdirectories)
if [[ -n "$dir_expr" ]]; then
    echo -e "\n${YELLOW}Searching for directories to delete...${NC}"
    while IFS= read -r -d '' dir; do
        safe_delete "$dir" "directory"
    done < <(find . -mindepth 1 -not -path . -type d \( "$dir_expr" \) -print0 2>/dev/null)
fi

# Remove files (exclude base directory but include all subdirectories)
if [[ -n "$file_expr" ]]; then
    echo -e "\n${YELLOW}Searching for files to delete...${NC}"
    while IFS= read -r -d '' file; do
        safe_delete "$file" "file"
    done < <(find . -mindepth 1 -not -path . -type f \( "$file_expr" \) -print0 2>/dev/null)
fi

echo -e "\n${GREEN}Cleanup completed!${NC}"
