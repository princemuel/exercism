export const square = (/** @type {number} */ n) => {
    if (n < 1 || n > 64) throw new Error("square must be between 1 and 64");
    return 2n ** BigInt(n - 1);
};

export const total = () => (1n << 64n) - 1n;
