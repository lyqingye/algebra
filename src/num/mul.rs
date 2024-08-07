use crate::num::limb::Limb;
use crate::num::uint::Uint;
use crate::num::wide::Wide;
use std::ops::Mul;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub(crate) fn overflowing_mul(&self, rhs: &Self) -> (Self, bool) {
        let r = self.split_mul(rhs);
        (r.low, r.high.is_nonzero())
    }

    #[inline(always)]
    pub(crate) fn split_mul(&self, rhs: &Self) -> Wide<LIMBS> {
        let mut temp = vec![Limb::ZERO; LIMBS * 2];

        for i in 0..LIMBS {
            let mut carry = Limb::ZERO;
            for j in 0..LIMBS {
                let (ret, c) = temp[i + j].mac(self.limbs[i], rhs.limbs[j], carry);
                carry = c;
                temp[i + j] = ret;
            }
            temp[i + LIMBS] = carry;
        }

        let mut low = [Limb::ZERO; LIMBS];
        low.copy_from_slice(&temp[0..LIMBS]);
        let mut high = [Limb::ZERO; LIMBS];
        high.copy_from_slice(&temp[LIMBS..LIMBS * 2]);

        Wide {
            low: Self { limbs: low },
            high: Self { limbs: high },
        }
    }

    #[inline(always)]
    pub(crate) fn wrapping_mul(&self, rhs: &Self) -> Self {
        self.split_mul(rhs).low
    }
}

impl<const LIMBS: usize> Mul<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;
    fn mul(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let (product, overflow) = self.overflowing_mul(rhs);
        assert!(!overflow, "attempted to multiple with overflow");
        product
    }
}

#[cfg(test)]
mod test {
    use crate::num::uint::U128;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_mul() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            let (c, overflow) = (a as u128).overflowing_mul(b as u128);
            if overflow {
                continue;
            }

            let a_1 = U128::from(a);
            let b_1 = U128::from(b);
            let c_1 = a_1 * &b_1;

            assert_eq!(U128::from_u128(c), c_1)
        }
    }

    #[test]
    fn test_split_mul() {
        let a = U128::from(230679353788795331459744549142118481455u128);
        let b = U128::from(146263473042228956998536595460379662786u128);
        let actual = a.split_mul(&b);
        assert_eq!(
            actual.low.to_string(),
            "157876027405260771784662058730631689886"
        );
        assert_eq!(
            actual.high.to_string(),
            "99152841064272478578353237181199359824"
        );
    }
}
