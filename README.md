# Experimenting with Finite Fields and Compilers

I have been experimenting with including the ASM written for the SQIsign C implementation into a Rust finite field type, and although the finite field performs much better at a very low level, the magic of the Rust compiler seems to have a hard time optimising complex operations (like 2D isogeny chains) and
the finite field made with the fp2 crate macro seems to be the fastest option.

There are two core finite fields in GF(p). `FpGen` is made with the `fp2::define_fp_core` macro. `GF5_248` is a custom type, which is really just the C implementation converted to Rust. (Actually, historically this was written in rust first then converted to C for the SQIsign impl...)

Then, within `GF5_248` we have either a native implementation or a implementation which directly calls `fp_asm_` methods compiled from the assembly in the SQIsign submission.

## GF(p) Benchmarks

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

### No feature flags

| Operation | no-asm | asm | fp2 crate |
|---|---|---|---|
| add | 13.51 | 16.31 | 9.42 |
| sub | 13.81 | 14.08 | 7.18 |
| mul | 161.30 | **30.03** | 56.10 |
| square | 106.75 | **33.10** | 73.04 |

Here, `asm` is the fastest for mul and square, but curiously the hand-written ASM for add and subtract is slower, which could be looked into for the C impl.


### Including `target-cpu=native`

| Operation | no-asm | asm | fp2 crate |
|---|---|---|---|
| add | 13.50 | 16.26 | 9.57 |
| sub | 13.80 | 14.14 | 7.20 |
| mul | 46.50 | **30.42** | 52.30 |
| square | 33.07 | 33.14 | 39.78 |

As expected, the performance of the asm implementation is unchanged but the gap between the fields is closed thanks to the use of the available x86_64 intrinsics

### Benchmarking Something Real

For my use case, I am not computing finite field arithmetic in vacuum, so there's a dummy implementation of a long 2D isogeny chain in the theta model. Benchmarking this, we find that the ASM implementation is the slowest option when `target-cpu=native` is enabled.


| Build | no-asm | asm | fp2 crate |
|---|---|---|---|
| No flags | 9,231,319 | **3,627,533** | 4,343,623 |
| target-cpu=native | **2,951,955** | 4,088,350 | **2,847,057** |

## GF(p^2) Benchmarks

A similar story happens for GF(p^2)

```
Using generated GF(p)

GF(p^2) mul:                 159.25  (58)
GF(p^2) square:              112.39  (64)
GF(p^2) 248-chain:       8,911,853.00  (206)
```

```
Benchmarking SQIsign impl with asm

GF(p^2) mul:                 147.55  (156)
GF(p^2) square:               98.36  (106)
GF(p^2) 248-chain:      10,577,974.60  (224)
```

```
Benchmarking SQIsign impl without asm

GF(p^2) mul:                 164.66  (59)
GF(p^2) square:              113.94  (212)
GF(p^2) 248-chain:       9,567,945.80  (220)
```

We find that the ASM single operations are the fastest (just) but LLVM is able to do more magic for long computations, like the isogeny chain
