#!/usr/bin/env bash

main() {
    local number="$1"

    result=""

    ((number % 3 == 0)) && result+="Pling"
    ((number % 5 == 0)) && result+="Plang"
    ((number % 7 == 0)) && result+="Plong"

    echo "${result:-$number}"
}

main "$@"
