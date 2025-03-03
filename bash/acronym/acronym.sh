#!/usr/bin/env bash

error() {
    printf '%s\n' "$*"
    exit 1
}

main() {
    (($# > 0)) || error "Usage: acronym.sh <string1> <string2> <stringn>"

    echo "$1" | tr -d "'" | tr -cs '[:alpha:]' '\n' | cut -c1 | tr '[:lower:]' '[:upper:]' | tr -d '\n'

}

main "$@"
