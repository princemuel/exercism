export function score(x: number, y: number): number {
    const distance = Math.hypot(x, y);
    const scores = new Map([
        [distance <= 10, 1],
        [distance <= 5, 5],
        [distance <= 1, 10],
    ]);
    return scores.get(true) ?? 0;
}
