use crate::num::limb::Limb;
use crate::num::uint::Uint;
use std::ops::Add;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub fn adc(&self, rhs: &Self, mut carry: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];

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
        assert_eq!(s, Limb::ZERO, "attempted to add with overflow");
        r
    }
}
