#!/usr/bin/env bash

main() {

    local input="$1"

    if command -v perl >/dev/null 2>&1; then
        # Use Perl for UTF-8-safe reversal
        printf %s "$input" | perl -CS -Mutf8 -ne 'chomp; print scalar reverse'
    else
        # Fallback: Bash byte-wise reversal (ASCII-safe only)
        local reversed=""
        local char
        while IFS= read -d "" -r -n 1 char; do
            reversed="$char$reversed"
        done < <(printf %s "$input")
        printf %s "$reversed"
    fi
}

main "$@"
