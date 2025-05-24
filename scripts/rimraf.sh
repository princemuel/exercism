#!/usr/bin/env bash
set -euo pipefail

# Configuration
DIRS_TO_DELETE=("target" "build" ".pytest_cache" "__pycache__" "node_modules" ".yarn" ".venv")
FILES_TO_DELETE=(".editorconfig")

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Function to safely delete items
safe_delete() {
    local item="$1"
    local type="$2"
    if [[ -e "$item" ]]; then
        echo -e "${YELLOW}Deleting $type:${NC} $item"
        rm -rf "$item"
        echo -e "${GREEN}✓ Deleted${NC}"
        return 0
    else
        echo -e "${BLUE}Item not found:${NC} $item"
        return 1
    fi
}

echo -e "${GREEN}Starting cleanup...${NC}"

# Debug: show what we're looking for
echo -e "\n${BLUE}Looking for directories:${NC} ${DIRS_TO_DELETE[*]}"
echo -e "${BLUE}Looking for files:${NC} ${FILES_TO_DELETE[*]}"

# Remove directories using simpler approach
echo -e "\n${YELLOW}Searching for directories to delete...${NC}"
found_dirs=0
for target_dir in "${DIRS_TO_DELETE[@]}"; do
    # Find in subdirectories only (not in root), any depth beyond first level
    while IFS= read -r -d '' dir; do
        # Skip if it's a direct child of current directory
        if [[ "$dir" =~ ^\./[^/]+$ ]]; then
            echo -e "${BLUE}Skipping root-level:${NC} $dir"
            continue
        fi
        safe_delete "$dir" "directory" && ((found_dirs++))
    done < <(find . -mindepth 1 -type d -name "$target_dir" -print0 2>/dev/null)
done

if [[ $found_dirs -eq 0 ]]; then
    echo -e "${BLUE}No target directories found${NC}"
fi

# Remove files using simpler approach
echo -e "\n${YELLOW}Searching for files to delete...${NC}"
found_files=0
for target_file in "${FILES_TO_DELETE[@]}"; do
    while IFS= read -r -d '' file; do
        # Skip if it's a direct child of current directory
        if [[ "$file" =~ ^\./[^/]+$ ]]; then
            echo -e "${BLUE}Skipping root-level:${NC} $file"
            continue
        fi
        safe_delete "$file" "file" && ((found_files++))
    done < <(find . -mindepth 1 -type f -name "$target_file" -print0 2>/dev/null)
done

if [[ $found_files -eq 0 ]]; then
    echo -e "${BLUE}No target files found${NC}"
fi

# Debug: show directory structure (first 2 levels)
echo -e "\n${BLUE}Current directory structure (first 2 levels):${NC}"
find . -maxdepth 2 -type d | head -20

echo -e "\n${GREEN}Cleanup completed!${NC}"
