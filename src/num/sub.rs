use crate::num::limb::Limb;
use crate::num::uint::Uint;
use std::ops::Sub;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub fn sbb(&self, rhs: &Self, mut borrow: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            let (w, b) = self.limbs[i].sbb(rhs.limbs[i], borrow);
            limbs[i] = w;
            borrow = b;
            i += 1;
        }

        #[cfg(test)]
        {
            if borrow == Limb::ZERO {
                assert_eq!(*self, *rhs + &Self { limbs });
            }
        }

        (Self { limbs }, borrow)
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
