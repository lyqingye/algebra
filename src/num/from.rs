use crate::num::limb::Limb;
use crate::num::uint::Uint;
impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub(crate) const fn from_u64(n: u64) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }

    #[inline(always)]
    pub(crate) const fn from_u128(n: u128) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as u64;
        limbs[1].0 = (n >> Limb::BITS) as u64;
        Self { limbs }
    }
}

impl<const LIMBS: usize> From<u64> for Uint<LIMBS> {
    fn from(n: u64) -> Self {
        Self::from_u64(n)
    }
}

impl<const LIMBS: usize> From<u128> for Uint<LIMBS> {
    fn from(n: u128) -> Self {
        Self::from_u128(n)
    }
}

#[cfg(test)]
mod test {
    use crate::num::uint::U128;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_from_u128() {
        let mut rng = thread_rng();
        for _ in 0..10000 {
            let v: u128 = rng.gen();
            let v2 = U128::from_u128(v);
            assert_eq!(v2.to_string(), v.to_string());
        }
    }
}
