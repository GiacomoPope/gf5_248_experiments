mod fp2_macro;
use gf5_248::FpGen;

mod use_sop {
    fp2::define_fp2_from_type!(
        typename = Fp2,
        base_field = super::FpGen,
        use_sum_of_products = true,
    );

    super::fp2_macro::define_fp2_bench!(Fp2, "Benchmarking GF(p) generated with SOP");
}

mod dont_use_sop {
    fp2::define_fp2_from_type!(
        typename = Fp2Alt,
        base_field = super::FpGen,
        use_sum_of_products = false,
    );

    super::fp2_macro::define_fp2_bench!(Fp2Alt, "Benchmarking GF(p) generated without SOP");
}

fn main() {
    use_sop::run_benchmarks();
    dont_use_sop::run_benchmarks();
}
