use crate::num::uint::Uint;
use std::ops::Rem;

impl<const LIMBS: usize> Rem<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;

    fn rem(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let (_q, r) = self.div_rem(rhs);
        r
    }
}
