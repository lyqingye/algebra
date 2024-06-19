use crate::exponent::fast_modular_exponentiation;

fn modular_exponentiation_with_fermat(a: u64, b: u64, p: u64) -> u64 {
    fast_modular_exponentiation(a, b % (p - 1), p)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modular_exponentiation_with_fermat() {
        assert_eq!(modular_exponentiation_with_fermat(2, 3, 5), 3);
        assert_eq!(modular_exponentiation_with_fermat(3, 13, 7), 3);
        assert_eq!(modular_exponentiation_with_fermat(5, 117, 19), 1);
        assert_eq!(modular_exponentiation_with_fermat(10, 0, 7), 1);
        assert_eq!(modular_exponentiation_with_fermat(7, 256, 13), 9);
        assert_eq!(
            modular_exponentiation_with_fermat(123456789, 123456789, 1000000007),
            907408795
        );
        assert_eq!(
            modular_exponentiation_with_fermat(987654321, 987654321, 1000000007),
            854621122
        );
    }
}
