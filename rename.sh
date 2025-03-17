#!/usr/bin/env bash

find . -type f -name "*.local.txt" -exec bash -c 'mv "$0" "${0%.local.txt}.txt"' {} \;
