class PrimeIterator implements Iterator<number> {
    private static readonly WHEEL = [4, 2, 4, 2, 4, 6, 2, 6];
    #primes: number[] = [];
    #candidate = 2;
    #wheelPosition = 0;
    #cache = new Map<number, boolean>();

    #isPrime(n: number): boolean {
        if (this.#cache.has(n)) return this.#cache.get(n)!;

        const sqrt = Math.floor(Math.sqrt(n));
        for (let divisor = 2; divisor <= sqrt; divisor++) {
            if (n % divisor === 0) {
                this.#cache.set(n, false);
                return false;
            }
        }
        this.#cache.set(n, true);
        return true;
    }

    public next(): IteratorResult<number, number> {
        let value: number;

        if (this.#primes.length === 0) {
            this.#primes.push(2);
            this.#candidate = 3;
            value = 2;
        } else if (this.#primes.length === 1) {
            this.#primes.push(3);
            this.#candidate = 5;
            value = 3;
        } else if (this.#primes.length === 2) {
            this.#primes.push(5);
            this.#candidate = 7;
            value = 5;
        } else {
            while (!this.#isPrime(this.#candidate)) {
                this.#candidate += PrimeIterator.WHEEL[this.#wheelPosition];
                this.#wheelPosition = (this.#wheelPosition + 1) % 8;
            }

            value = this.#candidate;
            this.#primes.push(value);

            this.#candidate += PrimeIterator.WHEEL[this.#wheelPosition];
            this.#wheelPosition = (this.#wheelPosition + 1) % 8;
        }

        return { value, done: false };
    }
}

export function nth(n: number): number {
    if (n < 1) throw new Error("Prime is not possible");
    const primes = new PrimeIterator();

    for (let i = 0; i < n - 1; i++) primes.next();
    return primes.next().value;
}
