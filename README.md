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

# Benchmarking GF(p^2) Arithmetic

## fp2 crate

### Using generic arithmetic

```
GF(p^2) add:                  16.54  (109)
GF(p^2) sub:                  12.00  (216)
GF(p^2) mul_small:            64.00  (139)
GF(p^2) mul:                 171.51  (110)
GF(p^2) square:              111.92  (68)
GF(p^2) div:                7902.15  (36)
GF(p^2) legendre:           6768.93  (9)
GF(p^2) sqrt:              39116.40  (136)
GF(p^2) 248-chain:       9227386.20  (226)
```

### Using specialised sums of products

```
GF(p^2) add:                  16.57  (246)
GF(p^2) sub:                  12.03  (249)
GF(p^2) mul_small:            88.96  (29)
GF(p^2) mul:                 159.18  (57)
GF(p^2) square:              110.96  (229)
GF(p^2) div:                7953.85  (244)
GF(p^2) legendre:           6796.14  (80)
GF(p^2) sqrt:              39142.80  (87)
GF(p^2) 248-chain:       8983611.10  (97)
```

## SQIsign (Pure Rust)

### Using generic arithmetic

```
GF(p^2) add:                  20.80  (31)
GF(p^2) sub:                  18.75  (200)
GF(p^2) mul_small:            43.21  (235)
GF(p^2) mul:                 174.81  (238)
GF(p^2) square:              104.15  (113)
GF(p^2) div:                8031.37  (3)
GF(p^2) legendre:           6745.70  (186)
GF(p^2) sqrt:              31782.20  (196)
GF(p^2) 248-chain:       9708372.80  (215)
```

### Using specialised sums of products

```
GF(p^2) add:                  19.01  (78)
GF(p^2) sub:                  18.75  (228)
GF(p^2) mul_small:            43.27  (210)
GF(p^2) mul:                 165.07  (168)
GF(p^2) square:              114.41  (86)
GF(p^2) div:                8028.10  (2)
GF(p^2) legendre:           6744.22  (41)
GF(p^2) sqrt:              31357.20  (182)
GF(p^2) 248-chain:       9554719.80  (204)
```

## SQIsign (ASM calls with extern)

### Using generic arithmetic

```
GF(p^2) add:                  20.53  (242)
GF(p^2) sub:                  18.88  (103)
GF(p^2) mul_small:            43.23  (29)
GF(p^2) mul:                 166.64  (75)
GF(p^2) square:               89.46  (136)
GF(p^2) div:                8039.99  (38)
GF(p^2) legendre:           6903.55  (24)
GF(p^2) sqrt:              33065.90  (127)
GF(p^2) 248-chain:       9271631.90  (244)
```

### Using specialised sums of products

```
GF(p^2) add:                  20.51  (163)
GF(p^2) sub:                  18.66  (194)
GF(p^2) mul_small:            43.12  (19)
GF(p^2) mul:                 147.42  (164)
GF(p^2) square:               89.19  (2)
GF(p^2) div:                8007.22  (74)
GF(p^2) legendre:           6895.92  (91)
GF(p^2) sqrt:              32269.10  (151)
GF(p^2) 248-chain:       8204013.90  (177)
```

## SQIsign (ASM calls with inline `asm!`)

### Using generic arithmetic

```
GF(p^2) add:                  20.82  (255)
GF(p^2) sub:                  18.76  (89)
GF(p^2) mul_small:            43.21  (250)
GF(p^2) mul:                 152.39  (183)
GF(p^2) square:               83.04  (197)
GF(p^2) div:                8011.91  (107)
GF(p^2) legendre:           6886.89  (176)
GF(p^2) sqrt:              35031.40  (179)
GF(p^2) 248-chain:       8697394.30  (227)
```

### Using specialised sums of products

This is not yet fully implemented, as we need to have inlined versions of some functions

```
GF(p^2) add:                  20.52  (103)
GF(p^2) sub:                  18.56  (215)
GF(p^2) mul_small:            43.22  (24)
GF(p^2) mul:                 148.38  (162)
GF(p^2) square:               82.93  (126)
GF(p^2) div:                8030.08  (39)
GF(p^2) legendre:           6897.80  (245)
GF(p^2) sqrt:              34030.00  (253)
GF(p^2) 248-chain:       7916787.60  (142)
```
