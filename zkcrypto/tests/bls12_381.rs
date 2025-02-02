#[cfg(test)]
pub mod tests {
    use kzg_bench::tests::bls12_381::*;
    use zkcrypto::kzg_types::{pairings_verify, ZkG1Projective, ZkG2Projective};
    use zkcrypto::utils::log_2_byte;
    use zkcrypto::zkfr::blsScalar;

    #[test]
    pub fn log_2_byte_works_() {
        log_2_byte_works(&log_2_byte);
    }

    #[test]
    pub fn fr_is_zero_works_() {
        fr_is_zero_works::<blsScalar>();
    }

    #[test]
    pub fn fr_is_one_works_() {
        fr_is_one_works::<blsScalar>();
    }

    #[test]
    fn fr_is_null_works_() {
        fr_is_null_works::<blsScalar>();
    }

    #[test]
    pub fn fr_from_uint64_works_() {
        fr_from_uint64_works::<blsScalar>();
    }

    #[test]
    pub fn fr_equal_works_() {
        fr_equal_works::<blsScalar>();
    }

    #[test]
    pub fn fr_negate_works_() {
        fr_negate_works::<blsScalar>();
    }

    #[test]
    pub fn fr_pow_works_() {
        fr_pow_works::<blsScalar>();
    }

    #[test]
    pub fn fr_div_works_() {
        fr_div_works::<blsScalar>();
    }

    #[test]
    pub fn fr_div_by_zero_() {
        fr_div_by_zero::<blsScalar>();
    }

    #[test]
    pub fn fr_uint64s_roundtrip_() {
        fr_uint64s_roundtrip::<blsScalar>();
    }

    #[test]
    pub fn p1_mul_works_() {
        p1_mul_works::<blsScalar, ZkG1Projective>();
    }

    #[test]
    pub fn p1_sub_works_() {
        p1_sub_works::<ZkG1Projective>();
    }

    #[test]
    pub fn p2_add_or_dbl_works_() {
        p2_add_or_dbl_works::<ZkG2Projective>();
    }

    #[test]
    pub fn p2_mul_works_() {
        p2_mul_works::<blsScalar, ZkG2Projective>();
    }

    #[test]
    pub fn p2_sub_works_() {
        p2_sub_works::<ZkG2Projective>();
    }

    #[test]
    pub fn g1_identity_is_infinity_() {
        g1_identity_is_infinity::<ZkG1Projective>();
    }

    #[test]
    pub fn g1_identity_is_identity_() {
        g1_identity_is_identity::<ZkG1Projective>();
    }

    //Siu dvieju testu nereikia nes nenaudojame g1_linear_combination
    // #[test]
    // pub fn g1_make_linear_combination_() {
    // g1_make_linear_combination::<blsScalar, ZkG1Projective>(&g1_linear_combination);
    // }

    // #[test]
    // pub fn g1_random_linear_combination_() {
    // g1_random_linear_combination::<blsScalar, ZkG1Projective>(&g1_linear_combination);
    // }

    #[test]
    pub fn pairings_work_() {
        pairings_work::<blsScalar, ZkG1Projective, ZkG2Projective>(&pairings_verify);
    }
}
