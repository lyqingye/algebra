fn fast_modular_exponentiation(_a: u64, _k: u64, m: u64) -> u64 {
    let mut a = _a % m;
    let mut k = _k;
    let mut result = 1;

    while k > 0 {
        if k & 1 == 1 {
            result = (result * a) % m;
        }
        a = (a * a) % m;

        k >>= 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fast_modular_exponentiation() {
        assert_eq!(fast_modular_exponentiation(2, 3, 5), 3);
        assert_eq!(fast_modular_exponentiation(3, 13, 7), 3);
        assert_eq!(fast_modular_exponentiation(5, 117, 19), 1);
        assert_eq!(fast_modular_exponentiation(10, 0, 7), 1);
        assert_eq!(fast_modular_exponentiation(7, 256, 13), 9);
        assert_eq!(
            fast_modular_exponentiation(123456789, 123456789, 1000000007),
            907408795
        );
        assert_eq!(
            fast_modular_exponentiation(987654321, 987654321, 1000000007),
            854621122
        );
    }
}
