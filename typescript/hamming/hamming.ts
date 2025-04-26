export function compute(left: string, right: string): number {
    if (left.length !== right.length)
        throw new Error("DNA strands must be of equal length.");

    return [...left].filter((char, i) => char !== right[i]).length;
}
