export function sum(factors: number[], limit: number): number {
    return [...range(1, limit)]
        .filter((n) => factors.some((factor) => n % factor === 0))
        .reduce((a, b) => (a += b), 0);
}

function* range(start: number, end: number, step = 1): Generator<number> {
    if (step === 0) throw new Error("step must not be 0");
    if (start < end) for (let i = start; i < end; i += step) yield i;
    else for (let i = start; i > end; i -= step) yield i;
}
