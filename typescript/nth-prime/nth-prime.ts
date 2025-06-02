const WHEEL = [4, 2, 4, 2, 4, 6, 2, 6];
const INITIAL_PRIMES = [2, 3, 5];

const primes = memoize(primeGenerator());

export function nth(n: number): number {
    if (n < 1) throw new Error("Prime is not possible");
    return primes(n - 1);
}

function* primeGenerator(): Generator<number, never, undefined> {
    yield* INITIAL_PRIMES;

    // Start wheel iteration from 7 (next candidate after 5 in INITIAL_PRIMES)
    let candidate = 7;
    let wheelPosition = 0;

    const cache = new Map<number, boolean>();
    const foundPrimes = [...INITIAL_PRIMES];

    function isPrime(n: number): boolean {
        if (cache.has(n)) return cache.get(n)!;

        const sqrt = Math.floor(Math.sqrt(n));

        for (const prime of foundPrimes) {
            if (prime > sqrt) break;
            if (n % prime === 0) {
                cache.set(n, false);
                return false;
            }
        }

        cache.set(n, true);
        return true;
    }

    while (true) {
        if (isPrime(candidate)) {
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
