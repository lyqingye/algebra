use crate::num::limb::Limb;
use rand_core::CryptoRngCore;

impl Limb {
    pub(crate) fn rand(rng: &mut impl CryptoRngCore) -> Self {
        Self(rng.next_u64())
    }
}
