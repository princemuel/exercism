from typing import List


def rebase(input_base: int, digits: List[int], output_base: int) -> List[int]:
    if input_base < 2:
        raise ValueError("input base must be >= 2")
    if output_base < 2:
        raise ValueError("output base must be >= 2")
    if any(d < 0 or d >= input_base for d in digits):
        raise ValueError("all digits must satisfy 0 <= d < input base")
    if not digits or all(d == 0 for d in digits):
        return [0]

    n = sum(d * (input_base**i) for i, d in enumerate(reversed(digits)))

    if n == 0:
        return [0]

    def to_base_digits(num: int, base: int):
        while num:
            yield num % base
            num //= base

    return list(reversed(tuple(to_base_digits(n, output_base))))
