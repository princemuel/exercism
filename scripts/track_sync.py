#!/usr/bin/env python3
"""
track_sync.py - Orchestrates the Exercism track management flow

This script coordinates the execution of track_stats.py and track_log.py
to ensure the track stats are updated before daily tracks are selected,
using the correct virtual environment.
"""

import subprocess
import sys
from pathlib import Path
from typing import List, Tuple

# Colorful output
COLORS = {
    "green": "\033[92m",
    "yellow": "\033[93m",
    "red": "\033[91m",
    "blue": "\033[94m",
    "bold": "\033[1m",
    "end": "\033[0m",
}


def kleur(text: str, color: str) -> str:
    """Add color to terminal text."""
    return f"{COLORS.get(color, '')}{text}{COLORS['end']}"


def get_venv_python() -> str:
    """Get the path to the Python interpreter in the virtual environment."""
    home_dir = Path.home()
    venv_python = home_dir / "exercism" / ".venv" / "bin" / "python"

    if not venv_python.exists():
        print(
            kleur(f"❌ Virtual environment Python not found at: {venv_python}", "red")
        )
        print("Make sure the virtual environment is set up correctly.")
        sys.exit(1)

    return str(venv_python)


def run_script(script_name: str, args: List[str] | None = None) -> Tuple[int, str]:
    """Run a Python script using the virtual environment Python."""
    if args is None:
        args = []

    script_path = Path(__file__).parent / script_name

    if not script_path.exists():
        return 1, f"Script not found: {script_path}"

    venv_python = get_venv_python()

    try:
        # Run in interactive mode (inherit stdin/stdout)
        cmd = [venv_python, str(script_path)] + args
        print(f"Running {kleur(script_name, 'blue')}...")

        result = subprocess.run(
            cmd,
            stdin=sys.stdin,  # Allow interactive input
            stdout=sys.stdout,  # Direct output to console
            stderr=subprocess.PIPE,  # Capture errors only
            text=True,
            check=False,
        )

        if result.returncode == 0:
            return 0, "Success"
        else:
            return result.returncode, f"Error: {result.stderr}"

    except Exception as e:
        return 1, f"Failed to execute {script_name}: {str(e)}"


def main() -> int:
    """Execute the track management scripts in the correct order."""
    print(kleur("🔄 Syncing Exercism track data", "bold"))

    # Step 1: Update track stats
    code, output = run_script("track_stats.py")
    if code != 0:
        print(kleur(f"❌ Failed to update track stats: {output}", "red"))
        return code

    print(output)
    print(kleur("✅ Track stats updated successfully", "green"))

    # Step 2: Generate daily track selection
    code, output = run_script("track_log.py")
    if code != 0:
        print(kleur(f"❌ Failed to select daily tracks: {output}", "red"))
        return code

    print(output)
    print(kleur("✅ Daily tracks selected successfully", "green"))

    print(kleur("\n🎉 Exercism track sync complete!", "bold"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
