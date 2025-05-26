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

const units = new Map([
    [1_000_000_000, "gigaohms"],
    [1_000_000, "megaohms"],
    [1_000, "kiloohms"],
]);

export function decodedResistorValue(colors: Color[]): string {
    const [first, second, third] = colors.slice(0, 3);
    const baseValue =
        ResistorValues.indexOf(first) * 10 + ResistorValues.indexOf(second);
    const multiplier = 10 ** ResistorValues.indexOf(third);
    const ohms = baseValue * multiplier;

    for (const [threshold, unit] of units.entries()) {
        if (ohms >= threshold) return `${ohms / threshold} ${unit}`;
    }

    return `${ohms} ohms`;
}
