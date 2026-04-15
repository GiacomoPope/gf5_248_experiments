mod fp2_macro;
use gf5_248::gf5_248::GF5_248;

mod use_sop {
    fp2::define_fp2_from_type!(
        typename = Fp2,
        base_field = super::GF5_248,
        use_sum_of_products = true,
    );

    #[cfg(feature = "asm")]
    super::fp2_macro::define_fp2_bench!(Fp2, "Benchmarking SQIsign impl with asm using SOP");

    #[cfg(not(feature = "asm"))]
    super::fp2_macro::define_fp2_bench!(Fp2, "Benchmarking SQIsign impl without asm using SOP");
}

mod dont_use_sop {
    fp2::define_fp2_from_type!(
        typename = Fp2Alt,
        base_field = super::GF5_248,
        use_sum_of_products = false,
    );

    #[cfg(feature = "asm")]
    super::fp2_macro::define_fp2_bench!(Fp2Alt, "Benchmarking SQIsign impl with asm no SOP");

    #[cfg(not(feature = "asm"))]
    super::fp2_macro::define_fp2_bench!(Fp2Alt, "Benchmarking SQIsign impl without asm no SOP");
}

fn main() {
    use_sop::run_benchmarks();
    dont_use_sop::run_benchmarks();
}
