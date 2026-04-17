mod fp2_macro;
use gf5_248::gf5_248::GF5_248;

mod use_sop {
    fp2::define_fp2_from_type!(
        typename = Fp2,
        base_field = super::GF5_248,
        use_sum_of_products = true,
    );

    #[cfg(all(target_arch = "x86_64", feature = "asm", not(feature = "asm-inline")))]
    super::fp2_macro::define_fp2_bench!(Fp2, "Benchmarking SQIsign impl with asm (SOP)");

    #[cfg(all(target_arch = "x86_64", feature = "asm-inline"))]
    super::fp2_macro::define_fp2_bench!(Fp2, "Benchmarking SQIsign impl with inlined asm (SOP)");

    #[cfg(not(any(feature = "asm", feature = "asm-inline")))]
    super::fp2_macro::define_fp2_bench!(Fp2, "Benchmarking SQIsign impl pure rust (SOP)");
}

mod dont_use_sop {
    fp2::define_fp2_from_type!(
        typename = Fp2Alt,
        base_field = super::GF5_248,
        use_sum_of_products = false,
    );

    #[cfg(all(target_arch = "x86_64", feature = "asm", not(feature = "asm-inline")))]
    super::fp2_macro::define_fp2_bench!(Fp2Alt, "Benchmarking SQIsign impl with asm (no SOP)");

    #[cfg(all(target_arch = "x86_64", feature = "asm-inline"))]
    super::fp2_macro::define_fp2_bench!(
        Fp2Alt,
        "Benchmarking SQIsign impl with inlined asm (no SOP)"
    );

    #[cfg(not(any(feature = "asm", feature = "asm-inline")))]
    super::fp2_macro::define_fp2_bench!(Fp2Alt, "Benchmarking SQIsign impl pure rust (no SOP)");
}

fn main() {
    use_sop::run_benchmarks();
    dont_use_sop::run_benchmarks();
}
