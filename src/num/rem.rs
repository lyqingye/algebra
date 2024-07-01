use std::ops::Rem;

use crate::num::uint::Uint;

impl<const LIMBS: usize> Rem<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;

    fn rem(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let (_q, r) = self.div_rem(rhs);
        r
    }
}

impl<const LIMBS: usize> Rem for Uint<LIMBS> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, r) = self.div_rem(&rhs);
        r
    }
}

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    /// self % 2^k
    pub(crate) fn rem_2k(&self, k: u32) -> Self {
        let bits = self.bits() as u32;
        if bits >= k {
            self.bitand(&(Self::ONE.wrapping_shl(k) - &Self::ONE))
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::num::uint::U128;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_rem_2k() {
        let mut rng = thread_rng();

        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let k = rng.gen_range(0..=127);
            let m = 1_u128 << k;
            let expect = U128::from_u128(a % m);
            let actual = U128::from_u128(a).rem_2k(k as u32);
            assert_eq!(expect, actual, "a: {} k: {} m: {}", a, k, m)
        }
    }
}
