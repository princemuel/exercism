pub fn factors(n: u64) -> Vec<u64> {
    let mut x = n;
    let mut divisors = Vec::with_capacity(32);
    let mut divisor = 2;

    while x > 1 {
        if x % divisor == 0 {
            divisors.push(divisor);
            x /= divisor;
        } else {
            divisor += 1;
        }
    }

    divisors
}
