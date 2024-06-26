use crate::num::limb::Limb;

#[derive(Copy, Clone, Eq, Debug)]
pub struct Uint<const LIMBS: usize> {
    pub(crate) limbs: [Limb; LIMBS],
}

impl<const LIMBS: usize> Uint<LIMBS> {
    pub const ZERO: Self = Self::from_u64(0);
    pub const ONE: Self = Self::from_u64(1);
    pub const TEN: Self = Self::from_u64(10);
    pub const MAX: Self = Self {
        limbs: [Limb::MAX; LIMBS],
    };

    #[inline(always)]
    pub(crate) fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&limb| limb == Limb::ZERO)
    }
}

pub type U128 = Uint<2>;