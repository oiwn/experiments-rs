mod primes;

use primes::*;

fn main() {
    // Test with N=1024 (2^10) and 30-bit primes
    let n = 1024;
    let logq = 30;

    println!("Finding NTT-friendly primes for N={}, {} bits:", n, logq);

    if let Some(prime) = naive::get_first_prime_down(logq, n) {
        println!("First prime found: {}", prime);
        println!("Is NTT-friendly: {}", core::is_ntt_friendly_prime(prime, n));

        if let Some(root) = core::find_primitive_root(prime, n) {
            println!("Primitive root: {}", root);
        }
    }

    // Get multiple primes
    let primes = naive::get_primes_down(logq, n, 3);
    println!("First 3 primes: {:?}", primes);
}
