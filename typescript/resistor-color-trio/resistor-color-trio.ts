type Color = (typeof ResistorValues)[number];

const ResistorValues = [
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "violet",
    "grey",
    "white",
] as const;

const SI_UNITS = [
    [1_000_000_000, "giga"],
    [1_000_000, "mega"],
    [1_000, "kilo"],
] as const;

export function decodedResistorValue(bands: Color[]): string {
    const [first, second, exponent] = bands
        .slice(0, 3)
        .map((color) => ResistorValues.indexOf(color));

    const resistance = (first * 10 + second) * 10 ** exponent;

    const [divisor, prefix] = SI_UNITS.find(
        ([threshold]) => resistance >= threshold,
    ) ?? [1, ""];

    return `${resistance / divisor} ${prefix}ohms`;
}
