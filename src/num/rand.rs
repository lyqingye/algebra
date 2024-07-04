use crate::num::limb::Limb;
use crate::num::uint::Uint;
use rand_core::CryptoRngCore;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub(crate) fn rand(rng: &mut impl CryptoRngCore) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];

        for limb in &mut limbs {
            *limb = Limb::rand(rng)
        }

        Self { limbs }
    }
}
