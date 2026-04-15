use gf5_248::gf5_248::GF5_248;
mod fp_macro;

#[cfg(feature = "asm")]
fp_macro::define_fp_bench!(GF5_248, "Benchmarking SQIsign impl with asm");

#[cfg(not(feature = "asm"))]
fp_macro::define_fp_bench!(GF5_248, "Benchmarking SQIsign impl without asm");
