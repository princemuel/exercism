#!/usr/bin/env bash

find . -name "target" -type d -exec rm -rf {} +
find . -name ".pytest_cache" -type d -exec rm -rf {} +
find . -name "__pycache__" -type d -exec rm -rf {} +
find . -name "node_modules" -type d -exec rm -rf {} +
find . -name ".yarn" -type d -exec rm -rf {} +
find . -name ".venv" -type d -exec rm -rf {} +
find . -name ".editorconfig" -type f -exec rm -rf {} +
