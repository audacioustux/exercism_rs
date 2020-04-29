pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];
    let mut factor: u64 = 2;

    let mut val = n;
    while val > 1 {
        while val % factor == 0 {
            prime_factors.push(factor);
            val /= factor;
        }
        factor += 1;
    }
    prime_factors
}
