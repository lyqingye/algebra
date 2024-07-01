use crate::num::uint::Uint;

#[inline(always)]
pub fn gcd<const LIMBS: usize>(a: &Uint<LIMBS>, b: &Uint<LIMBS>) -> Uint<LIMBS> {
    let mut a = *a;
    let mut b = *b;

    loop {
        if b.is_zero() {
            return a;
        } else {
            let temp = a % b;
            a = b;
            b = temp;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::num::gcd::gcd;
    use crate::num::uint::U128;

    #[test]
    fn test_gcd() {
        assert_eq!(
            U128::from(4u64),
            gcd(&U128::from(148u64), &U128::from(36u64))
        );
        assert_eq!(U128::from(5u64), gcd(&U128::from(15u64), &U128::from(5u64)));
    }
}
