#[allow(unused)]
macro_rules! define_fp_bench {
    ($Fp:ty, $label:expr) => {
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
            notimplemented!()
        }

        use gf5_248::isogeny_chain::{ThetaPoint, ThetaStructure};

        fn mkfp() -> $Fp {
            let mut buf = [0u8; (<$Fp>::ENCODED_LENGTH + 7) & !7usize];
            for i in 0..(buf.len() >> 3) {
                buf[(i << 3)..((i + 1) << 3)].copy_from_slice(&core_cycles().to_be_bytes());
            }
            <$Fp>::decode_reduce(&buf)
        }

        fn mkpt() -> ThetaPoint<$Fp> {
            let x = mkfp();
            let y = mkfp();
            let z = mkfp();
            let t = mkfp();
            ThetaPoint::new(&x, &y, &z, &t)
        }

        fn bench_fp_add() {
            let mut x = mkfp();
            let mut y = mkfp();
            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..1000 {
                    x += &y;
                    y += &x;
                    x += &y;
                    y += &x;
                    x += &y;
                    y += &x;
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) add:            {:13.2}  ({})",
                (tt[4] as f64) / 6000.0,
                x.encode()[0]
            );
        }

        fn bench_fp_sub() {
            let mut x = mkfp();
            let mut y = mkfp();
            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..1000 {
                    x -= &y;
                    y -= &x;
                    x -= &y;
                    y -= &x;
                    x -= &y;
                    y -= &x;
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) sub:            {:13.2}  ({})",
                (tt[4] as f64) / 6000.0,
                x.encode()[0]
            );
        }

        fn bench_fp_mul_small() {
            let mut x = mkfp();
            let mut tt = [0; 10];
            for i in 0..10 {
                let k = core_cycles() as i32;
                let begin = core_cycles();
                for _ in 0..1000 {
                    x.set_mul_small(k);
                    x.set_mul_small(k);
                    x.set_mul_small(k);
                    x.set_mul_small(k);
                    x.set_mul_small(k);
                    x.set_mul_small(k);
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) mul_small:      {:13.2}  ({})",
                (tt[4] as f64) / 6000.0,
                x.encode()[0]
            );
        }

        fn bench_fp_mul() {
            let mut x = mkfp();
            let mut y = mkfp();
            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..1000 {
                    x *= &y;
                    y *= &x;
                    x *= &y;
                    y *= &x;
                    x *= &y;
                    y *= &x;
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) mul:            {:13.2}  ({})",
                (tt[4] as f64) / 6000.0,
                x.encode()[0]
            );
        }

        fn bench_fp_square() {
            let mut x = mkfp();
            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..1000 {
                    x.set_square();
                    x.set_square();
                    x.set_square();
                    x.set_square();
                    x.set_square();
                    x.set_square();
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) square:         {:13.2}  ({})",
                (tt[4] as f64) / 6000.0,
                x.encode()[0]
            );
        }

        fn bench_sop_mul() {
            let mut x = mkfp();
            let mut y = mkfp();
            let mut z = mkfp();
            let mut t = mkfp();

            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..1000 {
                    x = <$Fp>::sum_of_products(&x, &y, &z, &t);
                    y = <$Fp>::sum_of_products(&x, &y, &z, &t);
                    z = <$Fp>::sum_of_products(&x, &y, &z, &t);
                    t = <$Fp>::sum_of_products(&x, &y, &z, &t);
                    x = <$Fp>::sum_of_products(&x, &y, &z, &t);
                    y = <$Fp>::sum_of_products(&x, &y, &z, &t);
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) sop:            {:13.2}  ({})",
                (tt[4] as f64) / 6000.0,
                x.encode()[0]
            );
        }

        fn bench_dop_mul() {
            let mut x = mkfp();
            let mut y = mkfp();
            let mut z = mkfp();
            let mut t = mkfp();

            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..1000 {
                    x = <$Fp>::difference_of_products(&x, &y, &z, &t);
                    y = <$Fp>::difference_of_products(&x, &y, &z, &t);
                    z = <$Fp>::difference_of_products(&x, &y, &z, &t);
                    t = <$Fp>::difference_of_products(&x, &y, &z, &t);
                    x = <$Fp>::difference_of_products(&x, &y, &z, &t);
                    y = <$Fp>::difference_of_products(&x, &y, &z, &t);
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) sop:            {:13.2}  ({})",
                (tt[4] as f64) / 6000.0,
                x.encode()[0]
            );
        }

        fn bench_fp_div() {
            let mut x = mkfp();
            let mut y = mkfp();
            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..100 {
                    x /= &y;
                    y /= &x;
                    x /= &y;
                    y /= &x;
                    x /= &y;
                    y /= &x;
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) div:            {:13.2}  ({})",
                (tt[4] as f64) / 600.0,
                x.encode()[0]
            );
        }

        fn bench_fp_legendre() {
            let mut x = mkfp();
            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..600 {
                    let ls = x.legendre();
                    x.set_cond(&x.mul2(), (ls >> 1) as u32);
                    x.set_cond(&x.mul3(), (-ls >> 1) as u32);
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) legendre:       {:13.2}  ({})",
                (tt[4] as f64) / 600.0,
                x.encode()[0]
            );
        }

        fn bench_fp_sqrt() {
            let mut x = mkfp();
            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..20 {
                    let (mut x2, r) = x.sqrt();
                    x2.set_cond(&<$Fp>::ONE, !r);
                    x += &x2;
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "GF(p) sqrt:           {:13.2}  ({})",
                (tt[4] as f64) / 20.0,
                x.encode()[0]
            );
        }

        fn bench_isogeny_chain() {
            let null = mkpt();
            let mut oa = ThetaStructure::new_from_point(&null);
            let k1 = mkpt();
            let k2 = mkpt();

            let mut tt = [0; 10];
            for i in 0..10 {
                let begin = core_cycles();
                for _ in 0..20 {
                    oa = oa.two_two_isogeny_chain(&k1, &k2, 248);
                }
                let end = core_cycles();
                tt[i] = end.wrapping_sub(begin);
            }
            tt.sort();
            println!(
                "(2^248, 2^248) chain: {:13.2}  ({})",
                (tt[4] as f64) / 20.0,
                oa.null_point().coords().0.encode()[0]
            );
        }

        fn main() {
            println!("{}", $label);
            println!("### len(p) = {}", <$Fp>::BIT_LENGTH);
            bench_fp_add();
            bench_fp_sub();
            bench_fp_mul_small();
            bench_fp_mul();
            bench_fp_square();
            bench_sop_mul();
            bench_dop_mul();
            bench_fp_div();
            bench_fp_legendre();
            bench_fp_sqrt();
            bench_isogeny_chain();
        }
    };
} // End of macro: define_fp_bench

#[allow(unused)]
pub(crate) use define_fp_bench;
