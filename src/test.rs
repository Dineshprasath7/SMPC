#[cfg(test)]
mod tests {
    use crate::shamir_secret_sharing::*;

    #[test]
    fn exp_log_round_trip() {
        for x in 1u8..=255 {
            assert_eq!(GF256_EXP[GF256_LOG[x as usize] as usize], x);
        }
    }

    #[test]
    fn multiplication_division_inverse() {
        for x in 1u8..=255 {
            assert_eq!(gf_div(gf_mul(x, 42), 42), x);
        }
    }

    #[test]
    fn multiplication_is_commutative() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                assert_eq!(gf_mul(a, b), gf_mul(b, a));
            }
        }
    }

    #[test]
    fn multiplication_identity() {
        for x in 0u8..=255 {
            assert_eq!(gf_mul(x, 1), x);
        }
    }

    #[test]
    fn multiplication_by_zero() {
        for x in 0u8..=255 {
            assert_eq!(gf_mul(x, 0), 0);
            assert_eq!(gf_mul(0, x), 0);
        }
    }

    #[test]
    fn division_identity() {
        for x in 1u8..=255 {
            assert_eq!(gf_div(x, 1), x);
        }
    }

    #[test]
    fn self_division() {
        for x in 1u8..=255 {
            assert_eq!(gf_div(x, x), 1);
        }
    }

    #[test]
    fn multiplication_associative() {
        for a in 1u8..20 {
            for b in 1u8..20 {
                for c in 1u8..20 {
                    let left = gf_mul(gf_mul(a, b), c);
                    let right = gf_mul(a, gf_mul(b, c));

                    assert_eq!(left, right);
                }
            }
        }
    }

    #[test]
    fn distributive_property() {
        for a in 1u8..20 {
            for b in 1u8..20 {
                for c in 1u8..20 {
                    let left = gf_mul(a, b ^ c);

                    let right = gf_mul(a, b) ^ gf_mul(a, c);

                    assert_eq!(left, right);
                }
            }
        }
    }
}
