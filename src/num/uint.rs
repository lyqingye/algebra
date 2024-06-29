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
    pub const BITS: usize = Limb::BITS * LIMBS;

    #[inline(always)]
    pub(crate) fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&limb| limb == Limb::ZERO)
    }

    #[inline(always)]
    pub(crate) fn is_nonzero(&self) -> bool {
        self.limbs.iter().any(|&limb| limb != Limb::ZERO)
    }
}

pub type U8192 = Uint<128>;
pub type U4096 = Uint<64>;
pub type U2048 = Uint<32>;
pub type U1024 = Uint<16>;
pub type U512 = Uint<8>;
pub type U256 = Uint<4>;
pub type U128 = Uint<2>;
pub type U64 = Uint<1>;
