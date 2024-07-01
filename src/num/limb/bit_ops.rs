use crate::num::limb::Limb;
use std::ops::{BitAnd, BitOr, Shl, Shr};

impl Limb {
    #[inline(always)]
    pub fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        let (r, o) = self.0.overflowing_shl(rhs);
        (Limb(r), o)
    }

    #[inline(always)]
    pub fn wrapping_shl(self, rhs: u32) -> Self {
        Limb(self.0.wrapping_shl(rhs))
    }

    #[inline(always)]
    pub fn wrapping_shr(self, rhs: u32) -> Self {
        Limb(self.0.wrapping_shr(rhs))
    }

    #[inline(always)]
    pub fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        let (r, o) = self.0.overflowing_shr(rhs);
        (Limb(r), o)
    }

    #[inline(always)]
    pub fn checked_shl(self, rhs: u32) -> Option<Self> {
        self.0.checked_shl(rhs).map(Limb)
    }

    #[inline(always)]
    pub fn checked_shr(self, rhs: u32) -> Option<Self> {
        self.0.checked_shr(rhs).map(Limb)
    }
}

impl Shl for Limb {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0.shl(rhs.0 as u32))
    }
}

impl Shr for Limb {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0.shr(rhs.0 as u32))
    }
}

impl BitOr for Limb {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
}

impl BitAnd for Limb {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0.bitand(rhs.0))
    }
}

#[cfg(test)]
mod test {
    use crate::num::limb::Limb;

    #[test]
    fn test_shr() {
        assert_eq!(Limb::ZERO, Limb(1u64).wrapping_shr(1));
        assert_eq!(Limb::ONE, Limb(0b10).wrapping_shr(1));
        assert_eq!(Limb::ONE, Limb(0b101).wrapping_shr(2));
        assert_eq!(Limb(0b11), Limb(u64::MAX).wrapping_shr(62));
    }
}
