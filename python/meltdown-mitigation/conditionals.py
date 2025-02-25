"""Functions to prevent a nuclear meltdown."""


def is_criticality_balanced(temperature: int | float, neutrons_emitted: int | float):
    """Verify criticality is balanced.

    :param temperature: int or float - temperature value in kelvin.
    :param neutrons_emitted: int or float - number of neutrons emitted per second.
    :return: bool - is criticality balanced?

    A reactor is said to be critical if it satisfies the following conditions:
    - The temperature is less than 800 K.
    - The number of neutrons emitted per second is greater than 500.
    - The product of temperature and neutrons emitted per second is less than 500000.
    """

    return (
        temperature < 800
        and neutrons_emitted > 500
        and (temperature * neutrons_emitted) < 500_000
    )


def reactor_efficiency(
    voltage: int | float, current: int | float, theoretical_max_power: int | float
):
    """Assess reactor efficiency zone.

    :param voltage: int or float - voltage value.
    :param current: int or float - current value.
    :param theoretical_max_power: int or float - power that corresponds to a 100% efficiency.
    :return: str - one of ('green', 'orange', 'red', or 'black').

    Efficiency can be grouped into 4 bands:

    1. green -> efficiency of 80% or more,
    2. orange -> efficiency of less than 80% but at least 60%,
    3. red -> efficiency below 60%, but still 30% or more,
    4. black ->  less than 30% efficient.

    The percentage value is calculated as
    (generated power/ theoretical max power)*100
    where generated power = voltage * current
    """

    efficiency = (voltage * current / theoretical_max_power) * 100

    # Define efficiency zones in a list using (10% steps, max index 10)
    zones = ["black"] * 3 + ["red"] * 3 + ["orange"] * 2 + ["green"] * 3

    # Use min to prevent out-of-bounds access for efficiency > 100
    return zones[min(int(efficiency // 10), 10)]


def fail_safe(
    temperature: int | float,
    neutrons_produced_per_second: int | float,
    threshold: int | float,
):
    """Assess and return status code for the reactor.

    :param temperature: int or float - value of the temperature in kelvin.
    :param neutrons_produced_per_second: int or float - neutron flux.
    :param threshold: int or float - threshold for category.
    :return: str - one of ('LOW', 'NORMAL', 'DANGER').

    1. 'LOW' -> `temperature * neutrons per second` < 90% of `threshold`
    2. 'NORMAL' -> `temperature * neutrons per second` +/- 10% of `threshold`
    3. 'DANGER' -> `temperature * neutrons per second` is not in the above-stated ranges
    """
    reactor_output = temperature * neutrons_produced_per_second

    low_threshold, high_threshold = 0.9 * threshold, 1.1 * threshold

    return {
        reactor_output < low_threshold: "LOW",
        low_threshold <= reactor_output <= high_threshold: "NORMAL",
    }.get(True, "DANGER")
