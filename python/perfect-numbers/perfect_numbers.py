import math
from typing import Set


def classify(number: int) -> str:
    """Classify a number as perfect, abundant, or deficient.

    A perfect number equals the sum of its positive divisors.
    An abundant number is less than the sum of its positive divisors.
    A deficient number is greater than the sum of its positive divisors.

    :param number: int a positive integer
    :return: str the classification of the input integer
    """
    is_positive_int(number)

    aliquot_sum = sum(aliquot_factors(number))

    if aliquot_sum == number:
        return "perfect"
    elif aliquot_sum > number:
        return "abundant"
    else:
        return "deficient"


def aliquot_factors(n: int) -> Set[int]:
    """Return the proper divisors of a number.

    :param n: int a positive integer
    :return: set the proper divisors of n (all divisors except n itself)
    """
    if n == 1:
        return set()

    start = 3 if n & 1 != 0 else 2  # being explicit here vs just n & 1
    stop = math.isqrt(n) + 1  # +1 to include the square root
    step = 2 if n & 1 != 0 else 1

    return {1} | {
        d for x in range(start, stop, step) if n % x == 0 for d in (x, n // x) if d < n
    }


def is_positive_int(n: int) -> None:
    if n < 1:
        raise ValueError("Classification is only possible for positive integers.")
