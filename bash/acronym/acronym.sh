#!/usr/bin/env bash

error() {
    printf '%s\n' "$*"
    exit 1
}

main() {
    (($# > 0)) || error "Usage: acronym.sh <string1> <string2> <stringn>"

    local phrase="$*"
    local acronym=""
    local spaced_phrase=${phrase//-/ }

    spaced_phrase=${spaced_phrase//\'/}
    cleaned_phrase=$(echo "$spaced_phrase" | tr -s "[:punct:][:space:]" " ")

    for word in $cleaned_phrase; do
        if [[ -n $word ]]; then
            local first_char="${word:0:1}"
            acronym+="${first_char^^}"
        fi
    done

    echo "$acronym"

}

main "$@"
