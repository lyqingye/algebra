use crate::num::limb::Limb;
use crate::num::uint::Uint;
use std::ops::Sub;

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
    pub fn sub_split(
        lhs_low: &Self,
        lhs_high: &Self,
        rhs_low: &Self,
        rhs_high: &Self,
    ) -> (Self, Self) {
        let (low, b) = lhs_low.sbb(rhs_low, Limb::ZERO);
        let (high, b2) = lhs_high.sbb(rhs_high, b);
        assert!(b2.is_zero(), "attempted to subtract with overflow");
        (low, high)
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
    use crate::num::limb::Limb;
    use crate::num::uint::{U128, U64};
    use rand::{thread_rng, Rng};
    use std::cmp::{max, min};

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
    fn test_sub_with_overflow() {
        let (_, overflow) = U128::ONE.sbb(&U128::MAX, Limb::ZERO);
        assert!(overflow.is_nonzero())
    }
    #[test]
    fn test_sub_split() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u128 = rng.gen();

            let a = max(a, b);
            let b = min(a, b);
            let c = a - b;

            const LOW_MASK: u128 = u64::MAX as u128;
            let a_high: u64 = (a >> 64) as u64;
            let b_high: u64 = (b >> 64) as u64;
            let c_high: u64 = (c >> 64) as u64;
            let a_low: u64 = (a & LOW_MASK) as u64;
            let b_low: u64 = (b & LOW_MASK) as u64;
            let c_low: u64 = (c & LOW_MASK) as u64;

            assert_eq!(((a_high as u128) << 64) + a_low as u128, a);
            assert_eq!(((b_high as u128) << 64) + b_low as u128, b);

            let (low, high) = U64::sub_split(
                &U64::from_u64(a_low),
                &U64::from_u64(a_high),
                &U64::from_u64(b_low),
                &U64::from_u64(b_high),
            );

            assert_eq!(U64::from_u64(c_low), low);
            assert_eq!(U64::from_u64(c_high), high);
        }
    }
}
