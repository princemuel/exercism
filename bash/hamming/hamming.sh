#!/usr/bin/env bash

error() {
    printf '%s\n' "$*"
    exit 1
}

main() {
    (($# == 2)) || error 'Usage: hamming.sh <string1> <string2>'

    local strand_a=$1
    local strand_b=$2

    # Check if the strands are of equal length
    ((${#strand_a} == ${#strand_a})) || error 'strands must be of equal length'

    local distance=0

    # Iterate over the characters of the strands
    for ((idx = 0; idx < ${#strand_a}; idx++)); do
        if [[ ${strand_a:idx:1} != "${strand_b:idx:1}" ]]; then
            ((distance++))
        fi
    done

    echo $distance
}

main "$@"
