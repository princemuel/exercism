import math


def score(x: int | float, y: int | float):
    distance = math.sqrt(x**2 + y**2)

    in_inner = distance <= 1
    in_middle = distance > 1 and distance <= 5
    in_outer = distance > 5 and distance <= 10

    return (10 * in_inner) + (5 * in_middle) + (1 * in_outer)
