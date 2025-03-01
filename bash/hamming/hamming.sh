#!/usr/bin/env bash

main() {
    if [[ $# -ne 2 ]]; then
        echo "Usage: hamming.sh <string1> <string2>"
        exit 1
    fi

    local strand_a=$1
    local strand_b=$2

    # Check if the strands are of equal length
    if [[ ${#strand_a} -ne ${#strand_b} ]]; then
        echo "strands must be of equal length"
        exit 1
    fi

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
