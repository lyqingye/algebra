use std::ops::Sub;

use crate::num::limb::Limb;
use crate::num::uint::Uint;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub fn sbb(&self, rhs: &Self, mut borrow: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];

        #[allow(clippy::needless_range_loop)]
        for i in 0..LIMBS {
            let (w, b) = self.limbs[i].sbb(rhs.limbs[i], borrow);
            limbs[i] = w;
            borrow = b;
        }

        (Self { limbs }, borrow)
    }

    #[inline(always)]
    pub fn wrapping_sub(&self, rhs: &Self) -> Self {
        let (r, _) = self.sbb(rhs, Limb::ZERO);
        r
    }
}

impl<const LIMBS: usize> Sub<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let (r, s) = self.sbb(rhs, Limb::ZERO);
        assert_eq!(s, Limb::ZERO, "attempted to subtract with overflow");
        r
    }
}

#[cfg(test)]
mod test {
    use std::cmp::{max, min};

    use rand::{thread_rng, Rng};

    use crate::num::limb::Limb;
    use crate::num::uint::U128;

    #[test]
    fn test_sub() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u128 = rng.gen();
            let a = max(a, b);
            let b = min(a, b);

            let ua = U128::from_u128(a);
            let ub = U128::from_u128(b);
            assert_eq!(U128::from_u128(a - b), ua - &ub);
        }
    }

    #[test]
    fn test_wrapping_sub() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u128 = rng.gen();

            let ua = U128::from_u128(a);
            let ub = U128::from_u128(b);
            assert_eq!(U128::from_u128(a.wrapping_sub(b)), ua.wrapping_sub(&ub));
        }
    }

    #[test]
    fn test_sub_with_overflow() {
        let (_, overflow) = U128::ONE.sbb(&U128::MAX, Limb::ZERO);
        assert!(overflow.is_nonzero())
    }
}
