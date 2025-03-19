"""Functions to help Azara and Rui locate pirate treasure."""

from typing import Tuple


def get_coordinate(record: Tuple[str, str]):
    """Return coordinate value from a tuple containing the treasure name, and treasure coordinate.

    :param record: tuple - with a (treasure, coordinate) pair.
    :return: str - the extracted map coordinate.
    """

    return record[1]


def convert_coordinate(coordinate: str):
    """Split the given coordinate into tuple containing its individual components.

    :param coordinate: str - a string map coordinate
    :return: tuple - the string coordinate split into its individual components.
    """

    return tuple(coordinate)


def compare_records(
    azara_record: Tuple[str, str], rui_record: Tuple[str, Tuple[str, str], str]
):
    """Compare two record types and determine if their coordinates match.

    :param azara_record: tuple - a (treasure, coordinate) pair.
    :param rui_record: tuple - a (location, tuple(coordinate_1, coordinate_2), quadrant) trio.
    :return: bool - do the coordinates match?
    """

    return tuple(azara_record[1]) == rui_record[1]


def create_record(
    azara_record: Tuple[str, str], rui_record: Tuple[str, Tuple[str, str], str]
):
    """Combine the two record types (if possible) and create a combined record group.

    :param azara_record: tuple - a (treasure, coordinate) pair.
    :param rui_record: tuple - a (location, coordinate, quadrant) trio.
    :return: tuple or str - the combined record (if compatible), or the string "not a match" (if incompatible).
    """

    return (
        azara_record + rui_record
        if compare_records(azara_record, rui_record)
        else "not a match"
    )


def clean_up(
    combined_record_group: Tuple[Tuple[str, str, str, Tuple[str, str], str]],
):
    """Clean up a combined record group into a multi-line string of single records.

    :param combined_record_group: tuple - everything from both participants.
    :return: str - everything "cleaned", excess coordinates and information are removed.

    The return statement should be a multi-lined string with items separated by newlines.

    (see HINTS.md for an example).
    """

    return "".join(
        [
            f"{(treasure, location, coords, quadrant)}\n"
            for (
                treasure,
                _,
                location,
                coords,
                quadrant,
            ) in combined_record_group
        ]
    )
