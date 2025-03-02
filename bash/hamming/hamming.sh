#!/usr/bin/env bash

error() {
    printf '%s\n' "$*"
    exit 1
}

cleanup() {
    rm -f file_a file_b
}

main() {
    (($# == 2)) || error 'Usage: hamming.sh <string1> <string2>'

    local strand_a=$1
    local strand_b=$2

    # Check if the strands are of equal length
    ((${#strand_a} == ${#strand_b})) || error 'strands must be of equal length'

    echo "$strand_a" >file_a
    echo "$strand_b" >file_b

    # Ensure cleanup happens on script exit
    trap cleanup EXIT

    cmp -l file_a file_b | wc -l
}

main "$@"
