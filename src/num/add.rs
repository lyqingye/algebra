use crate::num::limb::Limb;
use crate::num::uint::Uint;
use std::ops::Add;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub fn adc(&self, rhs: &Self, mut carry: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];

        #[allow(clippy::needless_range_loop)]
        for i in 0..LIMBS {
            let (w, c) = self.limbs[i].adc(rhs.limbs[i], carry);
            limbs[i] = w;
            carry = c;
        }

        (Self { limbs }, carry)
    }

    #[inline(always)]
    pub fn add_split(
        lhs_low: &Self,
        lhs_high: &Self,
        rhs_low: &Self,
        rhs_high: &Self,
    ) -> (Self, Self) {
        let (low, carry) = lhs_low.adc(rhs_low, Limb::ZERO);
        let (high, c2) = lhs_high.adc(rhs_high, carry);
        assert!(c2.is_zero(), "attempted to add with overflow");
        (low, high)
    }
}

impl<const LIMBS: usize> Add<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let (r, s) = self.adc(rhs, Limb::ZERO);
        assert!(s.is_zero(), "attempted to add with overflow");
        r
    }
}

#[cfg(test)]
mod test {
    use crate::num::limb::Limb;
    use crate::num::uint::{U128, U64};
    use rand::{thread_rng, Rng};

    #[test]
    fn test_add() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            let ua = U128::from_u64(a);
            let ub = U128::from_u64(b);
            assert_eq!(U128::from_u128(a as u128 + b as u128), ua + &ub);
        }
    }

    #[test]
    fn test_add_with_overflow() {
        let (_, overflow) = U128::MAX.adc(&U128::ONE, Limb::ZERO);
        assert!(overflow.is_nonzero())
    }
    #[test]
    fn test_add_split() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u128 = rng.gen();

            let (c, overflow) = a.overflowing_add(b);
            if overflow {
                continue;
            }

            const LOW_MASK: u128 = u64::MAX as u128;
            let a_high: u64 = (a >> 64) as u64;
            let b_high: u64 = (b >> 64) as u64;
            let c_high: u64 = (c >> 64) as u64;
            let a_low: u64 = (a & LOW_MASK) as u64;
            let b_low: u64 = (b & LOW_MASK) as u64;
            let c_low: u64 = (c & LOW_MASK) as u64;

            assert_eq!(((a_high as u128) << 64) + a_low as u128, a);
            assert_eq!(((b_high as u128) << 64) + b_low as u128, b);

            let (low, high) = U64::add_split(
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
