use crate::num::limb::Limb;
use crate::num::uint::Uint;
use std::ops::Div;

impl<const LIMBS: usize> Uint<LIMBS> {
    pub(crate) fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        assert_ne!(divisor, &Self::ZERO);

        let mut quotient = Self::ZERO;
        let mut d2 = *divisor;
        let mut pow = Self::ONE;

        loop {
            let (n, overflow) = d2.overflowing_shl(&Self::ONE);
            if overflow || (n > *self) {
                break;
            }
            d2 = n;
            pow = pow.wrapping_shl(&Self::ONE);
        }

        let mut remainder = *self;
        while remainder >= *divisor && d2 != Self::ZERO {
            let (r, borrow) = remainder.sbb(&d2, Limb::ZERO);
            if borrow == Limb::ZERO {
                remainder = r;
                quotient = quotient + &pow;
            }
            d2 = d2.wrapping_shr(&Self::ONE);
            pow = pow.wrapping_shr(&Self::ONE);
        }

        (quotient, remainder)
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