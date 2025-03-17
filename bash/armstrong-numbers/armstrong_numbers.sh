#!/usr/bin/env bash

error() {
    printf '%s\n' "$*" >&2
    exit 1
}

is_armstrong_number() {
    (($# > 0)) || error "Usage: is_armstrong_number <number>"

    local num=$1
    local length=${#num}
    local sum=0
    local temp=$num

    while [ "$temp" -gt 0 ]; do
        digit=$((temp % 10))
        sum=$((sum + digit ** length))
        temp=$((temp / 10))
    done

    if [ $sum -eq "$num" ]; then
        echo "true"
    else
        echo "false"
    fi
}

is_armstrong_number "$@"
