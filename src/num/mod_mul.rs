use crate::num::uint::Uint;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub(crate) fn mul_mod(&self, rhs: &Self, m: &Self) -> Self {
        self.split_mul(rhs).rem(m)
    }
}

#[cfg(test)]
mod test {
    use crate::num::uint::{U128, U64};
    use rand::{thread_rng, Rng};

    #[test]
    fn test_mul_mod() {
        let mut rng = thread_rng();
        let a: u64 = rng.gen();
        let b: u64 = rng.gen();
        let m: u64 = rng.gen_range(1u64..=u64::MAX);

        let expect = U64::from_u64((a as u128 * b as u128 % m as u128) as u64);
        let actual = U64::from_u64(a).mul_mod(&U64::from_u64(b), &U64::from_u64(m));
        assert_eq!(expect, actual)
    }

    #[test]
    fn test_mul_mod_large() {
        let m = U128::from(287215270712012985982119861826231487661u128);
        let a = U128::from(230679353788795331459744549142118481455u128);
        let b = U128::from(146263473042228956998536595460379662786u128);
        let rs = a.mul_mod(&b, &m);
        let expect = U128::from(202096250777435246437358086672401778403u128);
        assert_eq!(expect, rs)
    }
}
