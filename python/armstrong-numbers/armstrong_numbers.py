from math import log10


def is_armstrong_number(number: int):
    if number < 10:
        return True

    num_digits = int(log10(number)) + 1
    original = number
    sum = 0

    while number > 0:
        digit = number % 10
        sum += digit**num_digits
        number //= 10

    return sum == original
