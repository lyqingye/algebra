use std::cmp::{min, Ordering};

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

#[inline(always)]
pub fn binary_gcd<const LIMBS: usize>(a: &Uint<LIMBS>, b: &Uint<LIMBS>) -> Uint<LIMBS> {
    if a.is_zero() {
        return *b;
    }

    if b.is_zero() {
        return *a;
    }

    let mut u = *a;
    let mut v = *b;

    let i = u.trailing_zeros();
    let j = v.trailing_zeros();
    let k = min(i, j);

    u = u.wrapping_shr(i);
    v = v.wrapping_shr(j);

    loop {
        match u.cmp(&v) {
            Ordering::Greater => {
                u = u - &v;
                u = u.wrapping_shr(u.trailing_zeros());
            }
            Ordering::Less => {
                v = v - &u;
                v = v.wrapping_shr(v.trailing_zeros());
            }
            Ordering::Equal => break,
        };
    }

    u.wrapping_shl(k)
}

#[cfg(test)]
mod test {
    use crate::num::gcd::{binary_gcd, gcd};
    use crate::num::uint::U128;

    #[test]
    fn test_gcd() {
        assert_eq!(
            U128::from(4u64),
            gcd(&U128::from(148u64), &U128::from(36u64))
        );
        assert_eq!(U128::from(5u64), gcd(&U128::from(15u64), &U128::from(5u64)));
    }

    #[test]
    fn test_binary_gcd() {
        assert_eq!(
            U128::from(4u64),
            binary_gcd(&U128::from(148u64), &U128::from(36u64))
        );
        assert_eq!(
            U128::from(5u64),
            binary_gcd(&U128::from(15u64), &U128::from(5u64))
        );
    }
}
