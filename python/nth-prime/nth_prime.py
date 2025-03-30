from functools import cache


def prime(n: int) -> int:
    if n < 1:
        raise ValueError("there is no zeroth prime")
    prime_iter = PrimeIter()
    for _ in range(n - 1):
        next(prime_iter)
    return next(prime_iter)


class PrimeIter:
    WHEEL = [4, 2, 4, 2, 4, 6, 2, 6]

    def __init__(self):
        self.primes = []
        self.next_candidate = 2
        self.wheel_idx = 0

    @cache
    def is_prime(self, n: int) -> bool:
        return all(n % i != 0 for i in range(2, int(n**0.5) + 1))

    def __iter__(self):
        return self

    def __next__(self) -> int:
        if len(self.primes) == 0:
            self.primes.append(2)
            self.next_candidate = 3
            return 2
        elif len(self.primes) == 1:
            self.primes.append(3)
            self.next_candidate = 5
            return 3
        elif len(self.primes) == 2:
            self.primes.append(5)
            self.next_candidate = 7
            return 5

        while not self.is_prime(self.next_candidate):
            self.next_candidate += self.WHEEL[self.wheel_idx]
            self.wheel_idx = (self.wheel_idx + 1) % 8

        prime = self.next_candidate
        self.primes.append(prime)

        self.next_candidate += self.WHEEL[self.wheel_idx]
        self.wheel_idx = (self.wheel_idx + 1) % 8
        return prime
