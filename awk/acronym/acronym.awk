#!/usr/bin/env gawk -f

BEGIN { FPAT = "[[:alpha:]][[:alpha:]']*" }

{
    for (i = 1; i <= NF; i++) result = result substr($i, 1, 1)
}

END { print toupper(result) }
