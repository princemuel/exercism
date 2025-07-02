#!/usr/bin/env bash
set -euo pipefail

# Configuration
DIRS_TO_DELETE=("target" "build" ".pytest_cache" "__pycache__" "node_modules" ".yarn" ".venv" "bin" "dist" "coverage" ".mypy_cache" ".ruff_cache" ".idea" ".cache" "obj")
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

    # Skip .git directories
    if [[ "$item" == *"/.git" || "$item" == "./.git" ]]; then
        echo -e "${BLUE}Skipping .git:${NC} $item"
        return 1
    fi

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

# Remove directories
echo -e "\n${YELLOW}Searching for directories to delete...${NC}"
found_dirs=0
for target_dir in "${DIRS_TO_DELETE[@]}"; do
    while IFS= read -r -d '' dir; do
        safe_delete "$dir" "directory" && ((found_dirs++))
    done < <(find . -type d -name "$target_dir" -print0 2>/dev/null)
done

if [[ $found_dirs -eq 0 ]]; then
    echo -e "${BLUE}No target directories found${NC}"
fi

# Remove files
echo -e "\n${YELLOW}Searching for files to delete...${NC}"
found_files=0
for target_file in "${FILES_TO_DELETE[@]}"; do
    while IFS= read -r -d '' file; do
        safe_delete "$file" "file" && ((found_files++))
    done < <(find . -type f -name "$target_file" -print0 2>/dev/null)
done

if [[ $found_files -eq 0 ]]; then
    echo -e "${BLUE}No target files found${NC}"
fi

# Debug: show directory structure (first 2 levels)
echo -e "\n${BLUE}Current directory structure (first 2 levels):${NC}"
find . -type d | head -64

echo -e "\n${GREEN}Cleanup completed!${NC}"

# Reinstall root node_modules if package.json exists
if [[ -f "package.json" ]]; then
    echo -e "\n${YELLOW}Found package.json, reinstalling dependencies...${NC}"
    if command -v corepack >/dev/null 2>&1; then
        echo -e "${BLUE}Using corepack pnpm install${NC}"
        corepack pnpm install
        echo -e "${GREEN}✓ Dependencies reinstalled${NC}"
    else
        echo -e "${YELLOW}Warning: corepack not found, skipping dependency installation${NC}"
    fi
else
    echo -e "\n${BLUE}No package.json found, skipping dependency installation${NC}"
fi
