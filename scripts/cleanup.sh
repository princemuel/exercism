#!/bin/bash

# Exercism Exercise Cleanup Script
# Cleans up build artifacts, temporary files, and other cruft from exercism folders

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default exercism directory
EXERCISM_DIR="${HOME}/exercism"

# Allow override of exercism directory
if [[ $# -gt 0 ]]; then
    EXERCISM_DIR="$1"
fi

# Check if exercism directory exists
if [[ ! -d "$EXERCISM_DIR" ]]; then
    echo -e "${RED}Error: Exercism directory not found: $EXERCISM_DIR${NC}"
    echo "Usage: $0 [exercism_directory]"
    exit 1
fi

echo -e "${BLUE}🧹 Cleaning up exercism exercises in: $EXERCISM_DIR${NC}"

# Function to safely remove files/directories
safe_remove() {
    local target="$1"
    local description="$2"

    if [[ -e "$target" ]]; then
        rm -rf "$target"
        echo -e "${GREEN}  ✓ Removed $description${NC}"
        return 0
    fi
    return 1
}

# Function to clean a specific language directory
clean_language_dir() {
    local lang_dir="$1"
    local lang_name
    lang_name=$(basename "$lang_dir")

    echo -e "${YELLOW}Cleaning $lang_name exercises...${NC}"

    # Find all exercise directories (skip .git and other hidden dirs)
    find "$lang_dir" -mindepth 1 -maxdepth 1 -type d ! -name ".*" | while read -r exercise_dir; do
        local exercise_name
        exercise_name=$(basename "$exercise_dir")
        local cleaned_any=false

        pushd "$exercise_dir" >/dev/null 2>&1 || continue

        # Language-specific cleanup patterns
        case "$lang_name" in
        "cpp" | "c")
            safe_remove "*.o" "object files" && cleaned_any=true
            safe_remove "*.exe" "executables" && cleaned_any=true
            safe_remove "*.out" "output files" && cleaned_any=true
            safe_remove "a.out" "default executable" && cleaned_any=true
            safe_remove "core" "core dumps" && cleaned_any=true
            safe_remove ".deps" "dependency cache" && cleaned_any=true
            safe_remove ".cache" "dependency cache" && cleaned_any=true
            safe_remove "build/" "dependency cache" && cleaned_any=true
            ;;
        "x86-64-assembly" | "arm64-assembly")
            safe_remove "*.o" "object files" && cleaned_any=true
            safe_remove "*.exe" "executables" && cleaned_any=true
            safe_remove "*.out" "output files" && cleaned_any=true
            safe_remove "a.out" "default executable" && cleaned_any=true
            safe_remove "core" "core dumps" && cleaned_any=true
            safe_remove ".deps" "dependency cache" && cleaned_any=true
            safe_remove ".cache" "dependency cache" && cleaned_any=true
            safe_remove "build/" "dependency cache" && cleaned_any=true
            ;;
        "rust")
            safe_remove "target/" "build directory" && cleaned_any=true
            # safe_remove "Cargo.lock" "lock file" && cleaned_any=true
            ;;
        "go")
            safe_remove "*.exe" "executables" && cleaned_any=true
            safe_remove "go.sum" "module checksums" && cleaned_any=true
            safe_remove "go.mod" "module file" && cleaned_any=true
            ;;
        "javascript" | "typescript")
            safe_remove "node_modules/" "dependencies" && cleaned_any=true
            safe_remove ".yarn/" "dependencies" && cleaned_any=true
            # safe_remove "package-lock.json" "lock file" && cleaned_any=true
            # safe_remove "yarn.lock" "lock file" && cleaned_any=true
            safe_remove ".pnp.cjs" "lock file" && cleaned_any=true
            safe_remove ".pnp.loader.mjs" "lock file" && cleaned_any=true
            # safe_remove "pnpm-lock.yaml" "lock file" && cleaned_any=true
            safe_remove "dist/" "build output" && cleaned_any=true
            safe_remove "build/" "build directory" && cleaned_any=true
            safe_remove ".nyc_output/" "coverage cache" && cleaned_any=true
            safe_remove "coverage/" "coverage reports" && cleaned_any=true
            ;;
        "wasm")
            safe_remove "node_modules/" "dependencies" && cleaned_any=true
            safe_remove ".yarn/" "dependencies" && cleaned_any=true
            # safe_remove "package-lock.json" "lock file" && cleaned_any=true
            # safe_remove "yarn.lock" "lock file" && cleaned_any=true
            safe_remove ".pnp.cjs" "lock file" && cleaned_any=true
            safe_remove ".pnp.loader.mjs" "lock file" && cleaned_any=true
            # safe_remove "pnpm-lock.yaml" "lock file" && cleaned_any=true
            safe_remove "dist/" "build output" && cleaned_any=true
            safe_remove "build/" "build directory" && cleaned_any=true
            safe_remove ".nyc_output/" "coverage cache" && cleaned_any=true
            safe_remove "coverage/" "coverage reports" && cleaned_any=true
            ;;
        "python")
            safe_remove "__pycache__/" "Python cache" && cleaned_any=true
            safe_remove "*.pyc" "compiled Python files" && cleaned_any=true
            safe_remove "*.pyo" "optimized Python files" && cleaned_any=true
            safe_remove ".pytest_cache/" "pytest cache" && cleaned_any=true
            safe_remove ".coverage" "coverage data" && cleaned_any=true
            safe_remove "htmlcov/" "coverage HTML" && cleaned_any=true
            ;;
        "julia")
            safe_remove "Manifest.toml" "package manifest" && cleaned_any=true
            ;;
        "common-lisp")
            safe_remove "*.fasl" "compiled Lisp files" && cleaned_any=true
            safe_remove "*.dx64fsl" "compiled files" && cleaned_any=true
            safe_remove "*.lx64fsl" "compiled files" && cleaned_any=true
            ;;
        "csharp")
            safe_remove "bin/" "binary output" && cleaned_any=true
            safe_remove "obj/" "object files" && cleaned_any=true
            safe_remove "*.exe" "executables" && cleaned_any=true
            safe_remove "*.dll" "dynamic libraries" && cleaned_any=true
            safe_remove "packages/" "NuGet packages" && cleaned_any=true
            ;;
        "elixir")
            safe_remove "_build/" "build directory" && cleaned_any=true
            safe_remove "deps/" "dependencies" && cleaned_any=true
            safe_remove ".elixir-ls/" "editor generated" && cleaned_any=true
            safe_remove "*.beam" "compiled files" && cleaned_any=true
            safe_remove "mix.lock" "dependency lock" && cleaned_any=true
            ;;
        "ruby")
            safe_remove "*.gem" "gem files" && cleaned_any=true
            safe_remove ".bundle/" "bundler cache" && cleaned_any=true
            safe_remove "vendor/bundle/" "vendor gems" && cleaned_any=true
            safe_remove ".ruby-lsp/" "editor generated" && cleaned_any=true
            # safe_remove "Gemfile.lock" "gem lock file" && cleaned_any=true
            ;;
        "perl5")
            safe_remove "*.o" "object files" && cleaned_any=true
            safe_remove "blib/" "build library" && cleaned_any=true
            safe_remove "pm_to_blib" "build artifacts" && cleaned_any=true
            safe_remove "Makefile" "generated makefile" && cleaned_any=true
            safe_remove "Makefile.old" "old makefile" && cleaned_any=true
            ;;
        "bash" | "awk" | "jq")
            # Shell scripting languages - minimal cleanup needed
            safe_remove "*.log" "log files" && cleaned_any=true
            ;;
        "sqlite")
            safe_remove "*.db-shm" "shared memory files" && cleaned_any=true
            safe_remove "*.db-wal" "write-ahead log" && cleaned_any=true
            safe_remove "*.sqlite-shm" "shared memory files" && cleaned_any=true
            safe_remove "*.sqlite-wal" "write-ahead log" && cleaned_any=true
            safe_remove "output.json" "output log" && cleaned_any=true
            safe_remove "user-output.md" "write-ahead log" && cleaned_any=true
            ;;
        esac

        # Universal cleanup (applies to all languages)
        safe_remove ".DS_Store" "macOS metadata" && cleaned_any=true
        safe_remove "Thumbs.db" "Windows thumbnails" && cleaned_any=true
        safe_remove "*.swp" "vim swap files" && cleaned_any=true
        safe_remove "*.swo" "vim swap files" && cleaned_any=true
        safe_remove "*~" "backup files" && cleaned_any=true
        safe_remove ".#*" "emacs lock files" && cleaned_any=true
        safe_remove "*.tmp" "temporary files" && cleaned_any=true
        safe_remove "*.temp" "temporary files" && cleaned_any=true

        if [[ "$cleaned_any" == true ]]; then
            echo -e "${GREEN}  📁 Cleaned: $exercise_name${NC}"
        fi

        popd >/dev/null 2>&1
    done
}

# Track total space saved
initial_size=$(du -sb "$EXERCISM_DIR" 2>/dev/null | cut -f1 || echo "0")

# Clean each language directory
find "$EXERCISM_DIR" -mindepth 1 -maxdepth 1 -type d ! -name ".*" | while read -r lang_dir; do
    clean_language_dir "$lang_dir"
done

# Calculate space saved
final_size=$(du -sb "$EXERCISM_DIR" 2>/dev/null | cut -f1 || echo "0")
space_saved=$((initial_size - final_size))

if [[ $space_saved -gt 0 ]]; then
    # Convert bytes to human readable
    if [[ $space_saved -gt 1073741824 ]]; then
        space_saved_hr=$(echo "scale=2; $space_saved / 1073741824" | bc -l 2>/dev/null || echo "$(($space_saved / 1073741824))")
        unit="GB"
    elif [[ $space_saved -gt 1048576 ]]; then
        space_saved_hr=$(echo "scale=2; $space_saved / 1048576" | bc -l 2>/dev/null || echo "$(($space_saved / 1048576))")
        unit="MB"
    elif [[ $space_saved -gt 1024 ]]; then
        space_saved_hr=$(echo "scale=2; $space_saved / 1024" | bc -l 2>/dev/null || echo "$(($space_saved / 1024))")
        unit="KB"
    else
        space_saved_hr="$space_saved"
        unit="bytes"
    fi

    echo -e "${GREEN}🎉 Cleanup complete! Saved ${space_saved_hr} ${unit}${NC}"
else
    echo -e "${BLUE}✨ Everything was already clean!${NC}"
fi
