#!/usr/bin/env bash

if [[ $# -ne 1 ]] || ! [[ $1 =~ ^[[:digit:]]+$ ]]; then
    echo "Usage: $(basename "$0") <year>"
    exit 1
fi

year=$1

((!(year % 4) && year % 100 || !(year % 400))) && echo "true" || echo "false"
