#!/usr/bin/env bash

main() {
    local input="$1"

    # Convert to lowercase and extract only letters
    local letters=$(echo "$input" | tr '[:upper:]' '[:lower:]' | tr -cd 'a-z')

    # Sort and remove duplicates to get unique letters
    local unique_letters=$(echo "$letters" | fold -w1 | sort -u | tr -d '\n')

    # Check if we have all 26 letters
    [ ${#unique_letters} == 26 ] && echo "true" || echo "false"
}

main "$@"
