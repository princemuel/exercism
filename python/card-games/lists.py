"""Functions for tracking poker hands and assorted card tasks.

Python list documentation: https://docs.python.org/3/tutorial/datastructures.html
"""

from statistics import mean, median
from typing import List
from operator import concat


def get_rounds(number: int):
    """Create a list containing the current and next two round numbers.

    :param number: int - current round number.
    :return: list - current round and the two that follow.
    """

    return list(range(number, number + 3))


def concatenate_rounds(rounds_1: List[int], rounds_2: List[int]):
    """Concatenate two lists of round numbers.

    :param rounds_1: list - first rounds played.
    :param rounds_2: list - second set of rounds played.
    :return: list - all rounds played.
    """

    return concat(rounds_1, rounds_2)


def list_contains_round(rounds: List[int], number: int):
    """Check if the list of rounds contains the specified number.

    :param rounds: list - rounds played.
    :param number: int - round number.
    :return: bool - was the round played?
    """

    return number in rounds


def card_average(hand: List[int]):
    """Calculate and returns the average card value from the list.

    :param hand: list - cards in hand.
    :return: float - average value of the cards in the hand.
    """

    return mean(hand)


def approx_average_is_average(hand: List[int]):
    """Return if the (average of first and last card values) OR ('middle' card) == calculated average.

    :param hand: list - cards in hand.
    :return: bool - does one of the approximate averages equal the `true average`?
    """

    middle = median(hand)
    approx_mean = (hand[0] + hand[-1]) / 2
    real_mean = card_average(hand)

    return real_mean in (middle, approx_mean)


def average_even_is_average_odd(hand: List[int]):
    """Return if the (average of even indexed card values) == (average of odd indexed card values).

    :param hand: list - cards in hand.
    :return: bool - are even and odd averages equal?
    """
    # even_average = card_average(hand[::2])
    # odd_average = card_average(hand[1::2])

    return card_average(hand[::2]) == card_average(hand[1::2])


def maybe_double_last(hand: List[int]):
    """Multiply a Jack card value in the last index position by 2.

    :param hand: list - cards in hand.
    :return: list - hand with Jacks (if present) value doubled.
    """

    return (hand := hand[:-1] + [hand[-1] * 2] if hand[-1] == 11 else hand)
