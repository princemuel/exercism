from typing import List

Sides = List[int | float]


def equilateral(sides: Sides):
    a, b, c = sides
    return is_triangle(sides) and (a == b == c)


def isosceles(sides: Sides):
    a, b, c = sides
    return is_triangle(sides) and (a == b or a == c or b == c)


def scalene(sides: Sides):
    return is_triangle(sides) and not isosceles(sides)


def is_triangle(sides: Sides):
    a, b, c = sides
    return (a + b > c) and (a + c > b) and (b + c > a)
