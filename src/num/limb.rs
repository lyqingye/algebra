use std::cmp::Ordering;
use std::ops::{BitOr, Shl, Shr};

#[derive(Copy, Clone, Default, Debug, Eq)]
#[repr(transparent)]
pub struct Limb(pub u64);

impl Limb {
    pub const ZERO: Self = Limb(0u64);
    pub const ONE: Self = Limb(1u64);
    pub const MAX: Self = Limb(u64::MAX);
    pub const BITS: usize = 64;
    pub const BYTES: usize = 8;
}

impl Limb {
    #[inline(always)]
    pub fn adc(self, rhs: Self, carry: Self) -> (Self, Self) {
        let a = self.0 as u128;
        let b = rhs.0 as u128;
        let carry = carry.0 as u128;
        let ret = a + b + carry;
        (Self(ret as u64), Self((ret >> Self::BITS) as u64))
    }

    #[inline(always)]
    pub fn sbb(self, rhs: Self, borrow: Self) -> (Self, Self) {
        let a = self.0 as u128;
        let b = rhs.0 as u128;
        let borrow = (borrow.0 >> (Self::BITS - 1)) as u128;
        let ret = a.wrapping_sub(b + borrow);
        (Self(ret as u64), Self((ret >> Self::BITS) as u64))
    }

    #[inline(always)]
    /// Computes `self + (b * c) + carry`, returning the result along with the new carry.
    pub const fn mac(self, b: Limb, c: Limb, carry: Limb) -> (Limb, Limb) {
        let a = self.0 as u128;
        let b = b.0 as u128;
        let c = c.0 as u128;
        let carry = carry.0 as u128;
        let ret = a + (b * c) + carry;
        (Limb(ret as u64), Limb((ret >> Self::BITS) as u64))
    }

    pub fn overflowing_shl(self, shift: &Limb) -> (Limb, bool) {
        let (r, o) = self.0.overflowing_shl(shift.0 as u32);
        (Limb(r), o)
    }
}

impl From<usize> for Limb {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl PartialEq for Limb {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Ord for Limb {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Limb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Shl for Limb {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u128) << rhs.0) as u64)
    }
}

impl Shr for Limb {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u128) >> rhs.0) as u64)
    }
}

impl BitOr for Limb {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
}
