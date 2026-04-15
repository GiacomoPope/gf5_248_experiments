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
