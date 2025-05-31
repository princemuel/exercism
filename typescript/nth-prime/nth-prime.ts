class PrimeIterator implements Iterator<number>, Iterable<number> {
    private static readonly WHEEL = [4, 2, 4, 2, 4, 6, 2, 6];
    private static readonly INITIAL_PRIMES = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31,
    ];
    private static instance: PrimeIterator | null = null;

    #primes: number[] = [];
    #candidate: number;
    #wheelPosition = 0;
    #cache = new Map<number, boolean>();
    #currentIndex = 0;

    private constructor() {
        this.#primes = [...PrimeIterator.INITIAL_PRIMES];
        // Start wheel iteration from 37 (next candidate after 31)
        this.#candidate = 37;
    }

    #isPrime(n: number): boolean {
        if (this.#cache.has(n)) return this.#cache.get(n)!;
        // Use existing primes for faster divisibility testing
        const sqrt = Math.floor(Math.sqrt(n));
        for (const prime of this.#primes) {
            if (prime > sqrt) break;
            if (n % prime === 0) {
                this.#cache.set(n, false);
                return false;
            }
        }

        // Fall back to brute force if we don't have enough primes cached
        for (
            let divisor =
                this.#primes.length > 0
                    ? this.#primes[this.#primes.length - 1] + 1
                    : 2;
            divisor <= sqrt;
            divisor++
        ) {
            if (n % divisor === 0) {
                this.#cache.set(n, false);
                return false;
            }
        }

        this.#cache.set(n, true);
        return true;
    }

    public next(): IteratorResult<number, number> {
        if (this.#currentIndex < this.#primes.length) {
            return { value: this.#primes[this.#currentIndex++], done: false };
        }

        // Find next prime using wheel
        while (!this.#isPrime(this.#candidate)) {
            this.#candidate += PrimeIterator.WHEEL[this.#wheelPosition];
            this.#wheelPosition = (this.#wheelPosition + 1) % 8;
        }

        const value = this.#candidate;
        this.#primes.push(value);
        this.#currentIndex++;

        // Update candidate for next iteration
        this.#candidate += PrimeIterator.WHEEL[this.#wheelPosition];
        this.#wheelPosition = (this.#wheelPosition + 1) % 8;

        return { value, done: false };
    }

    [Symbol.iterator](): Iterator<number> {
        return this;
    }

    static getInstance(): PrimeIterator {
        return PrimeIterator.instance ?? new PrimeIterator();
    }

    static nthPrime(n: number): number {
        if (n < 1) throw new Error("Prime is not possible");

        const iterator = PrimeIterator.getInstance();

        if (n <= iterator.#primes.length) return iterator.#primes[n - 1];

        while (iterator.#primes.length < n) iterator.next();

        return iterator.#primes[n - 1];
    }

    static reset(): void {
        PrimeIterator.instance = null;
    }
}

export function nth(n: number): number {
    return PrimeIterator.nthPrime(n);
}
