use std::ops::Add;

use crate::num::limb::Limb;
use crate::num::uint::Uint;

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
    use rand::{thread_rng, Rng};

    use crate::num::limb::Limb;
    use crate::num::uint::U128;

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
}
