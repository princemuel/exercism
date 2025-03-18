from typing import List

UNEQUAL = 0
EQUAL = 1
SUBLIST = 2
SUPERLIST = 3


def sublist(a: List[int], b: List[int]):
    if a == b:
        return EQUAL
    if is_sublist(b, a):
        return SUPERLIST
    if is_sublist(a, b):
        return SUBLIST
    return UNEQUAL


def is_sublist(a: List[int], b: List[int]):
    x = len(a)
    y = len(b)
    return any(b[n : n + x] == a for n in range(y - x + 1))
