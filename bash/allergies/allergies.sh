#!/usr/bin/env bash

declare -a allergies=(
    "eggs"         # bit 0
    "peanuts"      # bit 1
    "shellfish"    # bit 2
    "strawberries" # bit 3
    "tomatoes"     # bit 4
    "chocolate"    # bit 5
    "pollen"       # bit 6
    "cats"         # bit 7
)

to_bool() {
    (($1 == 0)) && echo true || echo false
}

list() {
    local score=$1
    local allergies_list=()
    local bit_value=1

    score=$((score & 255))

    for allergen in "${allergies[@]}"; do
        if ((score & bit_value)); then
            allergies_list+=("$allergen")
        fi
        ((bit_value <<= 1))
    done

    if [ ${#allergies_list[@]} -gt 0 ]; then
        echo "${allergies_list[*]}"
    fi
}

allergic_to() {
    local allergen=$1
    local score=$2

    local allergy_list
    allergy_list=$(list "$score")

    [[ " $allergy_list " =~ $allergen ]]
    to_bool $?
}

main() {
    if (($# < 2)); then
        echo "Usage: $0 <score> <action> [allergen]" >&2
        echo "  score: numeric allergy score (0-255)" >&2
        echo "  action: 'list' or 'allergic_to'" >&2
        echo "  allergen: required for 'allergic_to' action" >&2
        echo "" >&2
        echo "Available allergens: ${allergies[*]}" >&2
        exit 1
    fi

    local score=$1
    local action=$2
    local allergen=$3

    if ! [[ "$score" =~ ^[0-9]+$ ]]; then
        echo "Error: Score must be a non-negative integer" >&2
        exit 1
    fi

    case "$action" in
    "allergic_to")
        if [[ -z "$allergen" ]]; then
            echo "Error: allergen name required for 'allergic_to' action" >&2
            exit 1
        fi
        allergic_to "$allergen" "$score"
        ;;
    "list")
        list "$score"
        ;;
    *)
        echo "Error: Unknown action '$action'" >&2
        echo "Valid actions: allergic_to, list" >&2
        exit 1
        ;;
    esac
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
