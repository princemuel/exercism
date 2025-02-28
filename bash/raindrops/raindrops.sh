#!/usr/bin/env bash

main() {
    local result=""

    (($1 % 3)) || result+="Pling"
    (($1 % 5)) || result+="Plang"
    (($1 % 7)) || result+="Plong"

    echo "${result:-$1}"
}

main "$@"
