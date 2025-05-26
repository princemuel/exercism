#!/usr/bin/env gawk -f

BEGIN {
    num % 3  || result = result "Pling"
    num % 5  || result = result "Plang"
    num % 7  || result = result "Plong"
    print result ? result : num
}
