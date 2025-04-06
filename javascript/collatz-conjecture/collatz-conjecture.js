export const steps = (/** @type {number} */ n) => {
    if (n <= 0 || !Number.isInteger(n))
        throw new TypeError("Only positive integers are allowed");

    let steps = 0;
    while (n != 1) {
        n = n % 2 ? 3 * n + 1 : n / 2;
        steps += 1;
    }

    return steps;
};
