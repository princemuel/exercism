#!/usr/bin/env bash

usage() {
    echo "Usage: error_handling.sh <person>"
    exit 1
}

main() {
    if [[ $# -ne 1 ]]; then
        usage
    fi
    echo "Hello, $1"
}

main "$@"
