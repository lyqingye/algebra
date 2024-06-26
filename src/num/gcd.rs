use crate::num::uint::Uint;

pub fn gcd<const LIMBS: usize>(a: &Uint<LIMBS>, b: &Uint<LIMBS>) -> Uint<LIMBS> {
    let mut a = *a;
    let mut b = *b;

    loop {
        if b == Uint::ZERO {
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
        assert_eq!(U128::from(4), gcd(&U128::from(148), &U128::from(36)));
        assert_eq!(U128::from(5), gcd(&U128::from(15), &U128::from(5)));
    }
}
