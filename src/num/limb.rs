mod bit_ops;
mod cmp;
mod fmt;
mod from;

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
}
