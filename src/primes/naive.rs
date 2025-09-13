use super::core::is_prime;

/// Finds the largest NTT-friendly prime ≤ 2^logq for a given ring dimension.
///
/// This function implements a naive downward search starting from the largest
/// number ≤ 2^logq that satisfies q ≡ 1 (mod 2N), then searches downward
/// in steps of 2N until a prime is found.
///
/// # Arguments
/// * `logq` - The logarithm of the target prime size (e.g., 30 for 30-bit primes)
/// * `n` - The ring dimension
///
/// # Returns
/// `Some(prime)` if found, `None` if no suitable prime exists
///
/// # Examples
/// ```
/// use experiments_rs::primes::naive::get_first_prime_down;
///
/// // Find largest 30-bit prime for N=1024
/// if let Some(prime) = get_first_prime_down(30, 1024) {
///     println!("Found prime: {}", prime);
/// }
/// ```
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

/// Finds multiple NTT-friendly primes ≤ 2^logq for a given ring dimension.
///
/// This function finds the largest prime using `get_first_prime_down` and then
/// continues searching downward to find additional primes.
///
/// # Arguments
/// * `logq` - The logarithm of the target prime size
/// * `n` - The ring dimension
/// * `count` - The number of primes to find
///
/// # Returns
/// A vector of NTT-friendly primes (may contain fewer than `count` primes
/// if not enough exist in the search space)
///
/// # Examples
/// ```
/// use experiments_rs::primes::naive::get_primes_down;
///
/// // Find 3 largest 30-bit primes for N=1024
/// let primes = get_primes_down(30, 1024, 3);
/// println!("Found primes: {:?}", primes);
/// ```
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
