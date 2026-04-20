use fp2::traits::Fq as FqTrait;
use gf5_248::gf5_248::GF5_248;
use gf5_248::FpGen;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use gf5_248::isogeny_chain::{ThetaPoint, ThetaStructure};

// Define the extensions
fp2::define_fp2_from_type!(
    typename = Fp2,
    base_field = GF5_248,
    use_sum_of_products = true,
);

fp2::define_fp2_from_type!(
    typename = Fp2Gen,
    base_field = FpGen,
    use_sum_of_products = true,
);

#[cfg(target_arch = "x86_64")]
pub fn core_cycles() -> u64 {
    use core::arch::x86_64::{_mm_lfence, _rdtsc};
    unsafe {
        _mm_lfence();
        _rdtsc()
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn core_cycles() -> u64 {
    unimplemented!()
}

fn mkfp<Fq: FqTrait>() -> Fq {
    let mut buf = [0u8; 64];
    for i in 0..(buf.len() >> 3) {
        buf[(i << 3)..((i + 1) << 3)].copy_from_slice(&core_cycles().to_be_bytes());
    }
    <Fq>::decode_reduce(&buf)
}

fn mkpt<Fq: FqTrait>() -> ThetaPoint<Fq> {
    ThetaPoint::new(&mkfp::<Fq>(), &mkfp::<Fq>(), &mkfp::<Fq>(), &mkfp::<Fq>())
}

macro_rules! define_chain_bench {
    ($Fq:ty, $fn_name:ident, $label:expr) => {
        fn $fn_name(c: &mut Criterion) {
            let null = mkpt::<$Fq>();
            let oa = ThetaStructure::new_from_point(&null);
            let k1 = mkpt::<$Fq>();
            let k2 = mkpt::<$Fq>();
            c.bench_function($label, |b| {
                b.iter(|| black_box(oa).two_two_isogeny_chain(&black_box(k1), &black_box(k2), black_box(248)))
            });
        }
    };
}

// ASM fields
#[cfg(all(target_arch = "x86_64", feature = "asm", not(feature = "asm-inline")))]
define_chain_bench!(GF5_248, bench_sqisign_fp, "GF(p) chain using SQIsign impl with extern asm");

#[cfg(all(target_arch = "x86_64", feature = "asm-inline"))]
define_chain_bench!(GF5_248, bench_sqisign_fp, "GF(p) chain using SQIsign impl with inlined asm");

#[cfg(not(any(feature = "asm", feature = "asm-inline")))]
define_chain_bench!(GF5_248, bench_sqisign_fp, "GF(p) chain using SQIsign impl with pure rust");

#[cfg(all(target_arch = "x86_64", feature = "asm", not(feature = "asm-inline")))]
define_chain_bench!(Fp2, bench_sqisign_fp2, "GF(p^2) chain using SQIsign impl with extern asm");

#[cfg(all(target_arch = "x86_64", feature = "asm-inline"))]
define_chain_bench!(Fp2, bench_sqisign_fp2, "GF(p^2) chain using SQIsign impl with inlined asm");

#[cfg(not(any(feature = "asm", feature = "asm-inline")))]
define_chain_bench!(Fp2, bench_sqisign_fp2, "GF(p^2) chain using SQIsign impl with pure rust");


// Generated fields
define_chain_bench!(FpGen, bench_generated_fp, "(2,2)-chain using generated impl");
define_chain_bench!(Fp2Gen, bench_generated_fp2, "(2,2)-chain using generated impl");


criterion_group!(benches, bench_sqisign_fp, bench_sqisign_fp2, bench_generated_fp, bench_generated_fp2);
criterion_main!(benches);