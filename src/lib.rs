pub mod gf5_248;
pub mod isogeny_chain;

// create a field using the fp2 macro
// SQISign level 1: p = 5 * 2^248 - 1
const MODULUS: [u64; 4] = [
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0x04FFFFFFFFFFFFFF,
];
fp2::define_fp_core!(typename = FpGen, modulus = MODULUS,);

#[cfg(test)]
mod test_sqisign_i_arithmetic {
    use super::FpGen;

    fp2::define_fp_tests!(FpGen);
}

#[cfg(test)]
mod test_gen_fp {
    use super::FpGen;

    fp2::define_fp_tests!(FpGen);
}

#[cfg(test)]
mod test_gen_fp2_with_sop {
    use super::FpGen;
    use super::MODULUS;

    fp2::define_fp2_from_type!(
        typename = Fp2Gen,
        base_field = FpGen,
        use_sum_of_products = true,
    );
    fp2::define_fp2_tests!(Fp2Gen, MODULUS, 5);
}


#[cfg(test)]
mod test_gen_fp2_without_sop {
    use super::FpGen;
    use super::MODULUS;

    fp2::define_fp2_from_type!(
        typename = Fp2Gen,
        base_field = FpGen,
        use_sum_of_products = false,
    );
    fp2::define_fp2_tests!(Fp2Gen, MODULUS, 5);
}

#[cfg(test)]
mod test_specific_fp2_with_sop {
    use crate::gf5_248::GF5_248;
    use super::MODULUS;

    fp2::define_fp2_from_type!(
        typename = Fp2Spec,
        base_field = GF5_248,
        use_sum_of_products = true,
    );
    fp2::define_fp2_tests!(Fp2Spec, MODULUS, 5);
}


#[cfg(test)]
mod test_specific_fp2_without_sop {
    use crate::gf5_248::GF5_248;
    use super::MODULUS;

    fp2::define_fp2_from_type!(
        typename = Fp2Spec,
        base_field = GF5_248,
        use_sum_of_products = false,
    );
    fp2::define_fp2_tests!(Fp2Spec, MODULUS, 5);
}