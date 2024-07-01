use std::ops::Div;

use crate::num::limb::Limb;
use crate::num::uint::Uint;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub(crate) fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        assert_ne!(rhs, &Self::ZERO);

        let divisor = *rhs;
        let mut bd = divisor.leading_zeros();
        let mut quo = Self::ZERO;
        let mut c = divisor.wrapping_shl(bd as u32);
        let mut rem = *self;

        loop {
            let (r, borrow) = rem.sbb(&c, Limb::ZERO);
            if borrow.is_zero() {
                rem = r;
                quo = quo.bitor(&Self::ONE)
            }
            if bd == 0 {
                break;
            }
            bd -= 1;
            c = c >> &Self::ONE;
            quo = quo << &Self::ONE;
        }

        #[cfg(test)]
        assert_eq!(*self, quo * &divisor + &rem);

        (quo, rem)
    }
}

impl<const LIMBS: usize> Div<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;
    fn div(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let (q, _r) = self.div_rem(rhs);
        q
    }
}

#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};

    use crate::num::uint::U128;

    #[test]
    fn test_div() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u64 = rng.gen();
            let ba = U128::from_u128(a);
            let bb = U128::from_u64(b);
            assert_eq!(
                (ba / &bb),
                U128::from_u128(a / b as u128),
                "a: {}, b: {}",
                a,
                b
            );
        }
    }
}
