/// Determines if a number is prime using trial division.
///
/// # Arguments
/// * `n` - The number to test for primality
///
/// # Returns
/// `true` if `n` is prime, `false` otherwise
///
/// # Examples
/// ```
/// use experiments_rs::primes::core::is_prime;
///
/// assert!(is_prime(2));
/// assert!(is_prime(17));
/// assert!(!is_prime(15));
/// ```
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Check odd divisors up to sqrt(n)
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

/// Modular exponentiation: computes `base^exp mod modulus` efficiently.
///
/// Uses the square-and-multiply algorithm with 128-bit arithmetic to prevent overflow.
///
/// # Arguments
/// * `base` - The base number
/// * `exp` - The exponent
/// * `modulus` - The modulus
///
/// # Returns
/// The result of `base^exp mod modulus`
fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        base = (base as u128 * base as u128 % modulus as u128) as u64;
        exp /= 2;
    }
    result
}

/// Checks if a number is an NTT-friendly prime.
///
/// An NTT-friendly prime must satisfy:
/// 1. `q` is prime
/// 2. `q ≡ 1 (mod 2N)` where `N` is the ring dimension
///
/// This condition ensures the existence of a primitive 2N-th root of unity
/// needed for the Number-Theoretic Transform.
///
/// # Arguments
/// * `q` - The candidate prime modulus
/// * `n` - The ring dimension (typically a power of two)
///
/// # Returns
/// `true` if `q` is an NTT-friendly prime, `false` otherwise
///
/// # Examples
/// ```
/// use experiments_rs::primes::core::is_ntt_friendly_prime;
///
/// // 1073707009 is prime and ≡ 1 mod 2048 (for N=1024)
/// assert!(is_ntt_friendly_prime(1073707009, 1024));
/// ```
pub fn is_ntt_friendly_prime(q: u64, n: u64) -> bool {
    if q < 2 || n < 1 {
        return false;
    }
    let two_n = 2 * n;
    q % two_n == 1 && is_prime(q)
}

/// Finds a primitive 2N-th root of unity modulo an NTT-friendly prime.
///
/// For an NTT-friendly prime `q` with `q ≡ 1 (mod 2N)`, this function
/// finds a primitive root `ψ` such that `ψ^(2N) ≡ 1 (mod q)` and
/// `ψ^m ≠ 1 (mod q)` for any `m < 2N`.
///
/// # Arguments
/// * `q` - The NTT-friendly prime modulus
/// * `n` - The ring dimension
///
/// # Returns
/// `Some(primitive_root)` if found, `None` if the input is not NTT-friendly
///
/// # Examples
/// ```
/// use experiments_rs::primes::core::find_primitive_root;
///
/// let q = 1073707009;
/// let n = 1024;
/// if let Some(root) = find_primitive_root(q, n) {
///     println!("Primitive root: {}", root);
/// }
/// ```
pub fn find_primitive_root(q: u64, n: u64) -> Option<u64> {
    if !is_ntt_friendly_prime(q, n) {
        return None;
    }

    let two_n = 2 * n;
    let phi = q - 1;

    // Find a generator of the multiplicative group
    for candidate in 2..q {
        if is_generator(candidate, q) {
            // The primitive 2N-th root is g^((q-1)/(2N))
            return Some(mod_exp(candidate, phi / two_n, q));
        }
    }
    None
}

/// Checks if a number is a generator of the multiplicative group modulo a prime.
///
/// A generator `g` modulo prime `p` satisfies that for every prime factor `f` of `p-1`,
/// `g^((p-1)/f) ≠ 1 (mod p)`.
///
/// # Arguments
/// * `g` - The candidate generator
/// * `p` - The prime modulus
///
/// # Returns
/// `true` if `g` is a generator, `false` otherwise
fn is_generator(g: u64, p: u64) -> bool {
    let phi = p - 1;

    // Check if g^((p-1)/prime_factor) != 1 for all prime factors
    let factors = prime_factors(phi);

    for factor in factors {
        if mod_exp(g, phi / factor, p) == 1 {
            return false;
        }
    }
    true
}

/// Computes the distinct prime factors of a number.
///
/// # Arguments
/// * `n` - The number to factor
///
/// # Returns
/// A vector containing the distinct prime factors of `n`
fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // Factor out 2
    while n % 2 == 0 {
        if !factors.contains(&2) {
            factors.push(2);
        }
        n /= 2;
    }

    // Factor out odd primes
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            if !factors.contains(&i) {
                factors.push(i);
            }
            n /= i;
        }
        i += 2;
    }

    // If n is prime
    if n > 1 && !factors.contains(&n) {
        factors.push(n);
    }

    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(17));
        assert!(is_prime(65537));

        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(15));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_is_ntt_friendly_prime() {
        // Known NTT-friendly prime for N=1024
        let q = 1073707009;
        let n = 1024;
        assert!(is_ntt_friendly_prime(q, n));

        // Not prime
        assert!(!is_ntt_friendly_prime(1073707008, n));

        // Not ≡ 1 mod 2N
        assert!(!is_ntt_friendly_prime(1073707010, n));
    }

    #[test]
    fn test_find_primitive_root() {
        let q = 1073707009;
        let n = 1024;

        if let Some(root) = find_primitive_root(q, n) {
            // Verify it's a 2N-th root of unity
            let two_n = 2 * n;
            assert_eq!(mod_exp(root, two_n, q), 1);

            // Verify it's primitive (not an m-th root for any m < 2N)
            // We'll just check a few divisors to keep the test fast
            for &divisor in &[2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
                if two_n % divisor == 0 {
                    assert_ne!(mod_exp(root, divisor, q), 1);
                }
            }
        } else {
            panic!("Should find primitive root for NTT-friendly prime");
        }
    }
}
