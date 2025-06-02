#!/usr/bin/env bash

if [[ $# -ne 1 ]]; then
    echo "Error: invalid input"
    exit 1
fi

n=$1

if [ "$n" = "total" ]; then
    printf "%llu\n" $(((((1 << 63) - 1) << 1) + 1))
    exit 0
fi

if ! [[ "$n" =~ ^[[:digit:]]+$ ]]; then
    echo "Error: invalid input"
    exit 1
fi

if [[ "$n" -lt 1 || "$n" -gt 64 ]]; then
    echo "Error: invalid input"
    exit 1
fi

printf "%llu\n" $((1 << (n - 1)))
