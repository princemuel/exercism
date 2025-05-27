#!/usr/bin/env bash

scheduler export tracks --format json --output tracks

mv "$HOME"/Downloads/tracks.json ./database/tracks.json

./scripts/readmes
