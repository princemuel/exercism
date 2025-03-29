pub fn nth(n: u32) -> u32 {
    PrimeIter::new().nth(n as usize).expect("There should always be an nth prime")
}

struct PrimeIter {
    primes: Vec<u32>,
    next_candidate: u32,
    wheel_index: usize,
}

impl PrimeIter {
    const WHEEL: [u32; 8] = [4, 2, 4, 2, 4, 6, 2, 6];

    fn new() -> Self {
        PrimeIter {
            primes: Vec::with_capacity(1024), // Larger initial capacity
            next_candidate: 2,
            wheel_index: 0,
        }
    }

    fn is_prime(&self, num: u32) -> bool {
        // Check divisibility by small primes first
        if num % 2 == 0 {
            return num == 2;
        }
        if num % 3 == 0 {
            return num == 3;
        }
        if num % 5 == 0 {
            return num == 5;
        }

        let sqrt_num = (num as f64).sqrt() as u32;
        for &p in &self.primes {
            if p > sqrt_num {
                break;
            }
            if num % p == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for PrimeIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Handle small primes specially
        match self.primes.len() {
            0 => {
                self.primes.push(2);
                self.next_candidate = 3;
                return Some(2);
            },
            1 => {
                self.primes.push(3);
                self.next_candidate = 5;
                return Some(3);
            },
            2 => {
                self.primes.push(5);
                self.next_candidate = 7;
                return Some(5);
            },
            _ => {},
        }

        while !self.is_prime(self.next_candidate) {
            self.next_candidate += Self::WHEEL[self.wheel_index];
            self.wheel_index = (self.wheel_index + 1) % 8;
        }

        let prime = self.next_candidate;
        self.primes.push(prime);
        self.next_candidate += Self::WHEEL[self.wheel_index];
        self.wheel_index = (self.wheel_index + 1) % 8;
        Some(prime)
    }
}
