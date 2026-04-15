use gf5_248::FpGen;
mod fp_macro;
fp_macro::define_fp_bench!(FpGen, "Benchmarking finite field from fp2 crate");
