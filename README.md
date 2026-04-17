# Experimenting with Finite Fields and Compilers

## Benchmarks

Benchmarks are done by counting clock cycles with the following function:

```rs
pub fn core_cycles() -> u64 {
    use core::arch::x86_64::{_mm_lfence, _rdtsc};
    unsafe {
        _mm_lfence();
        _rdtsc()
    }
}
```

So if you're working on a non-x86_64 machine this project wont work for you, sorry.

# Benchmarking GF(p) Arithmetic

We do micro benchmarks of GF(p) arithmetic as well as a dummy (2,2)-isogeny in the theta model of length 2^248. We find that there is rough parity between the GF(p) generated code and the specific code for the SQIsign prime. Using `extern` calls to the GF(p) ASM is much faster, but when the chain is computed the speed up is less than expected. By inlining the ASM things are even faster.

NOTE: If we use ASM for the addition and subtraction, things are MUCH slower for extern calls and I have not experimented yet with inlining the arithmetic.

## fp2 crate

GF(p) add:                     9.51  (129)
GF(p) sub:                     6.97  (56)
GF(p) mul_small:              48.35  (197)
GF(p) mul:                    51.79  (99)
GF(p) square:                 40.18  (129)
GF(p) sop:                    74.34  (14)
GF(p) sop:                    74.95  (237)
GF(p) div:                  7403.86  (208)
GF(p) legendre:             6675.56  (248)
GF(p) sqrt:                12234.80  (120)
GF(p) 248-chain:         2912838.40  (119)

## SQIsign (Pure Rust)

```
GF(p) add:                    13.50  (239)
GF(p) sub:                    13.81  (103)
GF(p) mul_small:              29.67  (144)
GF(p) mul:                    46.77  (249)
GF(p) square:                 32.74  (117)
GF(p) sop:                    64.86  (96)
GF(p) sop:                    67.27  (218)
GF(p) div:                  7513.46  (134)
GF(p) legendre:             6680.91  (94)
GF(p) sqrt:                 8234.80  (189)
GF(p) 248-chain:         2920229.10  (90)
```

## SQIsign (ASM calls with extern)

```
GF(p) add:                    13.48  (136)
GF(p) sub:                    13.83  (39)
GF(p) mul_small:              30.12  (196)
GF(p) mul:                    30.42  (83)
GF(p) square:                 32.52  (97)
GF(p) sop:                    70.86  (233)
GF(p) sop:                    75.13  (48)
GF(p) div:                  7514.84  (223)
GF(p) legendre:             6692.81  (153)
GF(p) sqrt:                 8895.30  (71)
GF(p) 248-chain:         2635559.40  (248)
```

## SQIsign (ASM calls with inline `asm!`)

```
GF(p) add:                    13.50  (244)
GF(p) sub:                    13.82  (142)
GF(p) mul_small:              30.10  (167)
GF(p) mul:                    30.91  (159)
GF(p) square:                 37.39  (36)
GF(p) sop:                    71.34  (121)
GF(p) sop:                    75.90  (79)
GF(p) div:                  7527.77  (4)
GF(p) legendre:             6709.99  (140)
GF(p) sqrt:                 9707.40  (211)
GF(p) 248-chain:         2363536.90  (214)
```
