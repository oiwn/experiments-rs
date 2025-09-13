# Generating NTT-Friendly Moduli for RNS-NTT

## Background and Requirements

In lattice-based fully homomorphic encryption (FHE) the ciphertext ring is
usually the negacyclic ring `R_q = Z_q[x]/(x^N + 1)`, where `N = 2^k` and `q` is
an integer modulus. To accelerate polynomial multiplication in this ring,
implementations use a double-CRT representation: coefficients are first
decomposed in a residue number system (RNS) basis of small moduli and then the
polynomials are transformed to the frequency domain with a Number-Theoretic
Transform (NTT). For the NTT to exist and be invertible the following must hold:

• **Ring dimension** – the degree `N` is usually a power of two.
• **Prime modulus** – each RNS modulus `q` must be prime to form the field `Z_q`.
  Composite moduli lack inverses and do not have primitive roots of unity, so NTTs
  cannot be performed.
• **Root-of-unity condition** – a primitive 2N-th root of unity must exist
  modulo q. Because the multiplicative group `F_q*` has order `q − 1`, we need 2N to
  divide q − 1. Equivalently, q ≡ 1 (mod 2N) ensures the existence of such roots.

These conditions mean that for an N-point NTT in the RNS basis one must generate
several distinct prime moduli `q_i` of similar bit-length such that q_i ≡ 1 (mod
2N). Cheon et al. demonstrate that in a full-RNS approximate FHE scheme each
modulus q_j is chosen close to a power of two and is congruent to one modulo 2N;
they give an explicit condition `|2^σ · q_j − 1| < 2^τ` and `q_j ≡ 1 (mod 2N)`.

OpenFHE's documentation similarly states that the library "supports any prime
modulus that is congruent to 1 mod 2 N" and that the implementation searches
through integers ≡ 1 (mod 2 N) until a prime is found. Lattigo exposes a
function GenerateNTTPrimes which "generates primes given logQ = size of the
primes, logN = size of N and level returning the appropriate primes with the
best available deviation from the base power of 2". Thus, prime moduli congruent
to 1 (mod 2N) are the de facto standard for RNS-NTT implementations.

In this report we discuss several methods for generating such NTT-friendly
moduli and outline how to implement and benchmark them in a small library.
The target moduli are 30–60 bit primes supporting NTTs of size 2^10 = 1024 up
to 2^13 = 8192. We also discuss whether composite co-prime moduli are viable
(they are not for NTT) and how special structured primes can improve reduction
performance.

## Why Primes Rather Than Generic Co-Primes?

At first glance one might consider using any pairwise co-prime moduli (composite
included) in the RNS representation. However, the NTT requires division by N and
the existence of a primitive 2N-th root of unity. For the ring R_q = Z_q[x]/(x^N
+ 1) with N = 2^k, the primitive root exists precisely when q ≡ 1 (mod 2N).
Composite moduli cannot satisfy this requirement because the multiplicative
group of a composite ring is not cyclic and roots of unity may not exist.
Furthermore, the NTT algorithm involves multiplying by 1/N in Z_q, which fails
if N shares factors with q. Thus, each modulus must be prime and congruent
to 1 (mod 2N). Using composite co-primes would break the invertibility of the
transform and is not used in modern FHE libraries.

## Existing Library Support

### OpenFHE

OpenFHE iterates through integers congruent to 1 mod 2N to find word-sized
primes. An answer on the OpenFHE forum explains that "for NTTs used in TFHE/FHE,
OpenFHE supports any prime modulus that is congruent to 1 mod 2 N" and that the
library's FirstPrime/LastPrime routines scan the arithmetic progression 1 + 2N ·
k until a prime is found.

### Lattigo

Lattigo (v4/v6) provides a ring.GenerateNTTPrimes function which generates
levels primes of bit-size logQ for a ring of size logN. The function returns the
primes "with the best available deviation from the base power of 2", meaning it
favours primes close to powers of two. In the Go examples, Lattigo automatically
picks moduli for given security levels by calling this function.

### Other Papers and Implementations

Cheon et al. describe a full RNS variant of approximate homomorphic encryption
and choose a ciphertext modulus as a product of distinct primes q_j satisfying
both a closeness to a power of two and the NTT condition q_j ≡ 1 (mod 2N).
They provide lists of 32–61 bit primes approximating powers of two for N =
2^15. A 2025 cryptography paper on encrypted control with Lattigo notes that
the ring parameters (N, q, σ) require N to be a power of two and "the modulus
q is a prime satisfying 1 ≡ q (mod 2N)", and example code shows the library
automatically selecting primes q = 72057594037616641 and P = 2251799813554177
meeting this condition for N = 2^13.

An interesting special case is the Goldilocks prime p = 2^64 − 2^32 + 1.
This 64-bit prime can be written as p = phi^2 − phi + 1 with phi = 2^32,
and it satisfies p − 1 = 2^32 · 3 · 5 · 17 · 257 · 65537. Consequently the
multiplicative group has a factor 2^32, allowing efficient NTTs of power-of-two
sizes. Because p fits in a 64-bit word, modular multiplication and reduction can
be extremely fast; however it offers only a single 64-bit modulus and is often
used as an optimisation in zero-knowledge proof systems rather than FHE.

## Methods for Generating NTT-Friendly Primes

Below we outline several practical algorithms for generating prime moduli q of
roughly 30–60 bits that satisfy q ≡ 1 (mod 2N). Each method can be encapsulated
as a module with functions to generate primes and to compute primitive roots
of unity. Benchmarks should measure generation time and NTT performance (e.g.
time-per-transform) using the generated moduli.

### 1. Randomised Search Over an Arithmetic Progression

**Idea.** Fix N and bit size B. Compute the modulus step M = 2N. Randomly select
odd integers k such that q = k · M + 1 has B bits; test q for primality using
a deterministic Miller–Rabin test for 64-bit numbers. Repeat until the required
number of primes is obtained. Randomised order avoids clustering near the start
of the range and spreads the search across the entire interval.

**Advantages.** Simple to implement; yields a variety of primes; can be tuned
to generate primes of arbitrary bit lengths. Suitable for generating chains for
deep RNS schemes.

**Implementation tips.** Pre-sieve the candidate k values by small prime
divisibility to avoid unnecessary primality tests. Use 128-bit arithmetic for
the primality test to avoid overflow. Example code is provided in the Rust and
Go snippets in the discussion above.

### 2. Deterministic Search Near a Power of Two

**Idea.** When you want primes as close as possible to 2^B, set an initial
candidate q0 = 2^B − c such that q0 ≡ 1 (mod 2N). Then decrement by multiples
of 2N until you find a prime. Continue the search downwards (or upwards) to
generate additional primes. Lattigo's GenerateNTTPrimes function follows this
pattern by returning primes that minimise the deviation from the closest power
of two.

**Advantages.** Generates primes with the maximal possible magnitude for
a given bit size, providing a larger noise budget in FHE. Deterministic
selection means repeated runs produce the same moduli, which aids debugging
and interoperability. Implementation is as simple as decrementing by 2N until a
prime is found.

**Example.** For N = 2048 (2N = 4096) and 60-bit primes, the search starts
at 2^60 minus the smallest c making 2^60 − c ≡ 1 (mod 4096), then tests each
candidate in steps of 4096 until it finds a prime. Cheon et al. list many 60-bit
primes of the form 0x7fffffff??0001 that satisfy q ≡ 1 (mod 2N).

### 3. Proth-type Prime Generation

**Idea.** When q has the form q = k · 2^m + 1 with k < 2^m, the modulus is a
Proth number. A Proth primality test states that if a^( (q − 1)/2 ) ≡ −1 (mod
q) for some integer a, then q is prime. Thus, for m ≥ log2(N) and small k, you
can quickly test whether k · 2^m + 1 is prime by a single exponentiation. If the
test fails, pick another k. Because 2N divides 2^m (when m ≥ log2(N)), this form
automatically satisfies q ≡ 1 (mod 2N).

**Advantages.** Provides provable primes rather than probable primes; extremely
fast primality test when the Proth condition holds; yields moduli close to
powers of two. Suitable when the bit length is at most 2m so that k < 2^m.

**Drawbacks.** Requires m to be at least half of the bit length of q, meaning 2N
must divide a relatively large power of two. In practice one often chooses m ≈
B/2 and adjusts N accordingly.

### 4. Pocklington–Lévy with Partial Factorisation

**Idea.** Write q − 1 = 2^m · r with known factorisation of the odd part r.
Factor r by trial division up to some bound; if you accumulate prime factors
whose product exceeds sqrt(q − 1) then Pocklington's theorem can certify that q
is prime. This approach yields provable primes even when k ≥ 2^m. It is similar
to Proth testing but requires partial factorisation of q − 1.

**Advantages.** Produces provable primes; can work with larger k than Proth
tests; tends to be fast when the odd cofactor has many small factors.

**Drawbacks.** Factorising the odd part can be expensive when it contains a
large prime factor. In the worst case it degenerates to a random search.

### 5. Structured Primes (Pseudo-Mersenne and Goldilocks)

**Idea.** Choose primes with a special algebraic form that simplifies modular
reduction. Examples include pseudo-Mersenne primes like 2^B − c and "Goldilocks"
primes such as 2^64 − 2^32 + 1. For Goldilocks, p − 1 factors as 2^32 · 3 · 5
· 17 · 257 · 65537, which provides a large power-of-two factor and small Fermat
prime factors; this allows efficient NTTs of power-of-two sizes without extra
twiddle factors.

**Implementation.** Search for primes of the form 2^B − c or 2^B + c with small
c that also satisfy q ≡ 1 (mod 2N). Because these primes are close to a power
of two, modular reduction can be implemented using shifts and additions rather
than long division. One can also use the fixed prime p = 2^64 − 2^32 + 1 when
a single 64-bit modulus suffices – it has a 2^32-th primitive root of unity.
However, for deep FHE one typically needs multiple primes, so this prime would
be part of a larger chain.

**Advantages.** Faster modular reduction and potentially faster NTTs.

**Drawbacks.** The search space is smaller; there may be fewer such primes in
the desired bit range; and their special structure could, in theory, lead to
unexpected algebraic attacks (none are currently known, but diversity in moduli
is prudent).

## Proposed Library Structure (Modules and Examples)

**random_ap_generator**
Functions to generate primes by randomised search in the progression 1 + 2N ·
k. Includes optional pre-sieving, deterministic Miller–Rabin, and collection of
multiple primes. generate_primes(n=2048, bits=60, count=4) returns four 60-bit
primes ≡ 1 (mod 4096).

**deterministic_search**
Functions to search downward (or upward) from 2^B in steps of 2N for primes.
Accepts bit size, ring dimension and number of primes. largest_primes(n=4096,
bits=61, levels=3) returns the three largest 61-bit primes satisfying q ≡ 1 (mod
8192).

**proth_generator**
Functions implementing the Proth test: given m and bit length, it picks random
k < 2^m, forms q = k · 2^m + 1 and applies the Proth primality test. Returns
prime if successful. proth_prime(m=20, bits=59) returns a 59-bit Proth prime
supporting NTTs up to 2^m.

**pocklington_generator**
Functions to generate primes by partial factorisation of q − 1 and
Pocklington–Lévy certification. Useful when k might be large.
pocklington_prime(n=2048, bits=58) returns a 58-bit proven prime ≡ 1 (mod 4096).

**structured_prime**
Functions to search for pseudo-Mersenne or Goldilocks-type primes that satisfy
q ≡ 1 (mod 2N). Includes hard-coded options like p = 2^64 − 2^32 + 1 and search
routines for 2^B − c. goldilocks_prime() returns the 64-bit Goldilocks prime;
search_pseudo_mersenne(n=1024, bits=60) finds a 60-bit prime of the form 2^60 −
c congruent to 1 mod 2048.

Each module should also include helper functions to compute primitive 2N-th and
N-th roots of unity. For a given prime q, one can find a primitive root g in
F_q* by testing candidates and derive psi = g^((q − 1)/(2N)) and omega = psi^2.

## Benchmarks

To compare the methods fairly, benchmark two aspects:

1. **Generation time** – measure the average time to find a prime using each
method for a fixed N and bit size. For random and deterministic searches, this
is the time to first prime; for Proth and Pocklington, it includes the primality
test and factorisation time.

2. **NTT throughput** – implement an in-place forward and inverse NTT using each
prime and measure cycles per coefficient. Structured primes may reduce reduction
costs; Proth primes behave like generic primes. Ensure that the same butterfly
implementation is used across experiments.

Use a range of ring sizes (1024, 2048, 4096 and 8192) and bit sizes (e.g. 59,
61 and 63) to cover typical FHE parameter sets. Record the number of candidate k
values tested and whether the prime is proven (Proth or Pocklington) or probable
(Miller–Rabin). Present results in prose and tables; avoid long sentences inside
tables as per the instructions.

## Conclusion

Efficient generation of NTT-friendly moduli is vital for fast RNS-NTT
implementations in FHE. The fundamental requirement is that each modulus q be
prime and satisfy q ≡ 1 (mod 2N). OpenFHE and Lattigo follow this by searching
arithmetic progressions until primes are found, while recent papers on full-RNS
variants choose primes close to powers of two and congruent to 1 (mod 2N).
Randomised and deterministic searches are simple and effective; Proth and
Pocklington tests produce provable primes; and structured primes like Goldilocks
offer special cases with very fast reduction. A modular library implementing and
benchmarking these methods will help practitioners choose the best approach for
their RNS-NTT systems.
