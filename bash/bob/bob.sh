#!/usr/bin/env bash

# Get the input from the first argument or read from stdin if no argument is provided
if [ $# -eq 0 ]; then
    read -r input
else
    input="$1"
fi

# Remove any trailing/leading whitespace
input=$(echo "$input" | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//')

# Check if the input is silence (empty or only whitespace)
if [ -z "$input" ]; then
    echo "Fine. Be that way!"
    exit 0
fi

# Check if the input is a question (ends with a question mark)
is_question=0
if [[ "$input" == *"?" ]]; then
    is_question=1
fi

# Check if the input is yelling (contains letters and all uppercase)
is_yelling=0
if [[ "$input" =~ [A-Z] && ! "$input" =~ [a-z] ]]; then
    is_yelling=1
fi

# Determine Bob's response based on the input type
if [ $is_question -eq 1 ] && [ $is_yelling -eq 1 ]; then
    echo "Calm down, I know what I'm doing!"
elif [ $is_question -eq 1 ]; then
    echo "Sure."
elif [ $is_yelling -eq 1 ]; then
    echo "Whoa, chill out!"
else
    echo "Whatever."
fi
