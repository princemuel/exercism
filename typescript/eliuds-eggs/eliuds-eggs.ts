export const eggCount = (x: number): number => {
    let count = 0;
    let n = x;
    while (n !== 0) {
        count += 1;
        n &= n - 1;
    }
    return count;
};
