pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(64);

    if n <= 1 {
        return result;
    }

    let mut partial_factors = factor_s(n);

    let mut i = 0;
    while i < partial_factors.len() {
        let factor = partial_factors[i];

        // Check if factor is prime using simplified Miller-Rabin
        if factor < 1_000_000 || is_probably_prime(factor) {
            result.push(factor);
            i += 1;
        } else {
            // Factor is composite, apply Pollard's rho
            let p = pollards_rho(factor);
            let q = factor / p;

            // Replace the composite factor with its factors
            partial_factors[i] = p;
            partial_factors.push(q);
            // Don't increment i, so we'll check p in the next iteration
        }
    }

    result.sort();
    result
}

pub fn factor_s(n: u64) -> Vec<u64> {
    if n <= 1 {
        return Vec::new();
    }

    let mut factors = Vec::with_capacity(64);
    let mut remaining = n;

    while remaining % 2 == 0 {
        factors.push(2);
        remaining /= 2;
    }

    while remaining % 3 == 0 {
        factors.push(3);
        remaining /= 3;
    }

    // Wheel factorization (based on 6k±1 pattern). skips nums divisible by 2 or 3
    let mut divisor = 5;
    let mut wheel_jump = 2; // Alternates between 2 and 4

    while divisor * divisor <= remaining {
        while remaining % divisor == 0 {
            factors.push(divisor);
            remaining /= divisor;
        }

        divisor += wheel_jump;
        wheel_jump = 6 - wheel_jump; // Alternates between 2 and 4
    }

    if remaining > 1 {
        factors.push(remaining);
    }

    factors
}

// For very large numbers, use a simple implementation of Pollard's rho
fn pollards_rho(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }

    let mut x = 2;
    let mut y = 2;
    let mut d = 1;

    // f(x) = (x² + 1) mod n
    let f = |x: u64| -> u64 {
        let x_squared = x.wrapping_mul(x);
        (x_squared.wrapping_add(1)) % n
    };

    // Floyd's cycle-finding algorithm
    while d == 1 {
        x = f(x);
        y = f(f(y)); // Move y twice as fast as x

        // Calculate gcd(|x-y|, n)
        d = gcd(x.abs_diff(y), n);
    }

    if d == n {
        return n;
    }

    d
}

// Euclidean algorithm for greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

const fn is_probably_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }

        i += 6;
    }
    true
}

// pub fn factors(mut n: u64) -> Vec<u64> {
//     let mut factors = Vec::with_capacity(16);
//     let mut candidates = (3..).step_by(2);
//     let mut prime = 2;

//     while n > 1 {
//         while n % prime == 0 {
//             n /= prime;
//             factors.push(prime);
//         }

//         prime = candidates.next().unwrap();
//     }

//     factors
// }
