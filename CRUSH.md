# Rust Experiments Codebase Guide

## Build & Test Commands
- `cargo build` - Build the project
- `cargo run` - Build and run
- `cargo test` - Run all tests
- `cargo test -- --nocapture` - Run tests with output
- `cargo test test_name` - Run specific test
- `cargo fmt --all` - Format code
- `cargo clippy` - Lint code
- `cargo check` - Type check without building

## Code Style Guidelines
- **Formatting**: Use `cargo fmt -all` (Rustfmt)
- **Linting**: Use `cargo clippy` for additional checks
- **Imports**: Group std, external, and local imports
- **Naming**: snake_case for variables/functions, PascalCase for types
- **Error Handling**: Use Result and Option types, prefer `?` operator
- **Documentation**: Use `///` for public items, `//!` for module docs

## Project Structure
- Rust 2024 edition
- Single binary crate
- Source in `src/main.rs`
- Cargo.toml for dependencies

## Development Workflow
1. Write code
2. `cargo fmt`
3. `cargo clippy`
4. `cargo test`
5. Commit

## Current Task: NTT Prime Generation Implementation

### Core Functions to Implement:
- `is_prime(n: u64) -> bool` - Miller-Rabin primality test
- `is_ntt_friendly_prime(q: u64, n: u64) -> bool` - Check q ≡ 1 (mod 2N) and primality
- `find_primitive_root(q: u64, n: u64) -> Option<u64>` - Find primitive 2N-th root

### Reference Implementation First:
- `get_first_prime_down(logq: u32, n: u64) -> u64` - Start from 2^logq and search downward
- All code organized in `primes/` module
- Focus on getting basic generation working before adding methods

### Implementation Order:
1. Basic prime testing infrastructure (is_prime, is_ntt_friendly_prime)
2. Reference downward search method (get_first_prime_down)
3. Primitive root finding
4. Then add other methods incrementally

### File Organization:
- Keep all prime-related code in `primes/` module
- Start with minimal set: core functions + one reference method
- Add other methods as separate files later
