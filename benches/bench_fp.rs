use gf5_248::gf5_248::GF5_248;
mod fp_macro;

#[cfg(all(target_arch = "x86_64", feature = "asm", not(feature = "asm-inline")))]
fp_macro::define_fp_bench!(GF5_248, "Benchmarking SQIsign impl with asm");

#[cfg(all(target_arch = "x86_64", feature = "asm-inline"))]
fp_macro::define_fp_bench!(GF5_248, "Benchmarking SQIsign impl with inlined asm");

#[cfg(not(any(feature = "asm", feature = "asm-inline")))]
fp_macro::define_fp_bench!(GF5_248, "Benchmarking SQIsign impl pure rust");
