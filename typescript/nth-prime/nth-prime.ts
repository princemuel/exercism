// Wheel of Eratosthenes - skips multiples of 2, 3, and 5
// Pattern: adds [4,2,4,2,4,6,2,6] cyclically starting from 7
const WHEEL = [4, 2, 4, 2, 4, 6, 2, 6];

const primes = memoize(primeGenerator());

export function nth(n: number): number {
    if (n < 1) throw new Error("Prime is not possible");
    return primes(n - 1); // Convert to 0-based indexing
}

function* primeGenerator(): Generator<number, never, undefined> {
    const foundPrimes = [2, 3, 5];
    yield* foundPrimes;

    let candidate = 7;
    let wheelPosition = 0;

    while (true) {
        const sqrt = Math.floor(Math.sqrt(candidate));
        let isPrime = true;

        // Test divisibility only up to √candidate
        for (const prime of foundPrimes) {
            if (prime > sqrt) break;
            if (candidate % prime === 0) {
                isPrime = false;
                break;
            }
        }

        if (isPrime) {
            foundPrimes.push(candidate);
            yield candidate;
        }

        candidate += WHEEL[wheelPosition];
        wheelPosition = (wheelPosition + 1) % WHEEL.length;
    }
}

function memoize<T>(gen: Generator<T, never, undefined>): (idx: number) => T {
    const cache: T[] = [];

    return (idx: number): T => {
        if (idx < 0) throw new Error("Index out of bounds");

        while (cache.length <= idx) cache.push(gen.next().value);

        return cache[idx];
    };
}
