#include "collatz_conjecture.h"
#include <stddef.h>
#include <limits.h>

int steps(int start)
{
    if (start < 1)
        return ERROR_VALUE;

    int step_count = 0;
    long long n = (long long)start;

    while (n != 1)
    {
        // Bitwise check for even
        if ((n & 1) == 0)
            n >>= 1;
        else
        {
            if (n > (LLONG_MAX - 1) / 3)
            {
                return ERROR_VALUE;
            }

            n = 3 * n + 1;
        }
        step_count++;

        if (step_count >= INT_MAX || n <= 0)
        {
            return ERROR_VALUE;
        }
    }

    return step_count;
}
