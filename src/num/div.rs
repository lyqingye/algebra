use crate::num::limb::Limb;
use crate::num::uint::Uint;
use std::ops::Div;

impl<const LIMBS: usize> Uint<LIMBS> {
    pub(crate) fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        assert_ne!(rhs, &Self::ZERO);

        let divisor = *rhs;
        let divisor_bits = divisor.bits();
        let bd = Self::BITS - divisor_bits;
        let mut quo = Self::ZERO;

        let mut c = divisor.wrapping_shl(bd as u32);
        let mut pow = Self::ONE.wrapping_shl(bd as u32);
        let mut rem = *self;

        while rem >= divisor && c.is_nonzero() {
            let (r, borrow) = rem.sbb(&c, Limb::ZERO);
            if borrow == Limb::ZERO {
                rem = r;
                quo = quo + &pow;
            }
            c = c.wrapping_shr(1);
            pow = pow.wrapping_shr(1);
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
    use crate::num::uint::U128;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_div() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u64 = rng.gen();
            let ba = U128::from_u128(a);
            let bb = U128::from_u64(b);
            println!("a: {}, b: {}", a, b);
            assert_eq!(
                (ba / &bb).to_string(),
                (a / b as u128).to_string(),
                "a: {}, b: {}",
                a,
                b
            );
            println!("{:}", ba / &bb);
        }
    }
}
