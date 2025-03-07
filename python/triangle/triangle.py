from typing import Callable, List

Sides = List[int | float]


def is_triangle(func: Callable) -> Callable:
    def inner(sides: Sides):
        return sum(sides) > 2 * max(sides) and func(sides)

    return inner


@is_triangle
def equilateral(sides: Sides):
    return len(set(sides)) == 1


@is_triangle
def isosceles(sides: Sides):
    return len(set(sides)) < 3


@is_triangle
def scalene(sides: Sides):
    return len(set(sides)) == 3
