use super::core::is_prime;

pub fn get_first_prime_down(logq: u32, n: u64) -> Option<u64> {
    if logq < 2 || n < 1 {
        return None;
    }

    let two_n = 2 * n;
    let target_bits = 1u64 << logq;

    // Find the largest number <= target_bits that is ≡ 1 (mod 2N)
    let mut candidate = target_bits - (target_bits % two_n) + 1;
    if candidate > target_bits {
        candidate -= two_n;
    }

    // Search downward until we find a prime
    while candidate > 1 {
        if is_prime(candidate) {
            return Some(candidate);
        }
        candidate -= two_n;
    }

    None
}

pub fn get_primes_down(logq: u32, n: u64, count: usize) -> Vec<u64> {
    let mut primes = Vec::new();

    if let Some(first_prime) = get_first_prime_down(logq, n) {
        primes.push(first_prime);

        let two_n = 2 * n;
        let mut candidate = first_prime - two_n;

        while primes.len() < count && candidate > 1 {
            if is_prime(candidate) {
                primes.push(candidate);
            }
            candidate -= two_n;
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_prime_down() {
        let prime = get_first_prime_down(30, 1024).unwrap();
        assert_eq!(prime, 1073707009);
        assert!(is_prime(prime));
        assert_eq!(prime % (2 * 1024), 1);
    }

    #[test]
    fn test_get_primes_down() {
        let primes = get_primes_down(30, 1024, 3);
        assert_eq!(primes.len(), 3);

        for &prime in &primes {
            assert!(is_prime(prime));
            assert_eq!(prime % (2 * 1024), 1);
        }
    }
}
