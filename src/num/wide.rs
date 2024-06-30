use crate::num::limb::Limb;
use crate::num::uint::Uint;
use std::ops::BitOr;

#[derive(Copy, Clone)]
pub struct Wide<const LIMBS: usize> {
    pub(crate) low: Uint<LIMBS>,
    pub(crate) high: Uint<LIMBS>,
}

impl<const LIMBS: usize> Wide<LIMBS> {
    pub const ZERO: Self = Self {
        low: Uint::ZERO,
        high: Uint::ZERO,
    };
    #[inline(always)]
    pub fn sub(&self, rhs: &Self) -> Self {
        let (r, borrow) = self.sbb(rhs, Limb::ZERO);
        assert!(borrow.is_zero(), "attempted to subtract with overflow");
        r
    }

    #[inline(always)]
    pub fn sbb(&self, rhs: &Self, borrow: Limb) -> (Self, Limb) {
        let (low, borrow) = self.low.sbb(&rhs.low, borrow);
        let (high, borrow) = self.high.sbb(&rhs.high, borrow);
        (Self { low, high }, borrow)
    }

    #[inline(always)]
    pub fn adc(&self, rhs: &Self, carry: Limb) -> (Self, Limb) {
        let (low, carry) = self.low.adc(&rhs.low, carry);
        let (high, carry) = self.high.adc(&rhs.high, carry);
        (Self { low, high }, carry)
    }
    #[inline(always)]
    pub fn add(&self, rhs: &Self) -> Self {
        let (r, carry) = self.adc(rhs, Limb::ZERO);
        assert!(carry.is_zero(), "attempted to add with overflow");
        r
    }

    #[inline(always)]
    pub fn shr1(&self) -> Self {
        self.shr(1)
    }

    #[inline(always)]
    pub fn shr(&self, shift: u32) -> Self {
        if shift == 0 {
            return *self;
        }

        let new_limbs: usize = LIMBS * 2;

        let mut limbs = vec![Limb::ZERO; new_limbs];
        let lhs = [self.low.limbs, self.high.limbs].concat();
        let shift_bit = shift as usize;

        let shift_num = shift_bit / Limb::BITS;
        let shr_shift = (shift_bit % Limb::BITS) as u32;
        let shl_shift = Limb::BITS as u32 - shr_shift;

        let mut high = Limb::ZERO;
        for i in (shift_num..new_limbs).rev() {
            let low = lhs[i].wrapping_shr(shr_shift);
            limbs[i - shift_num] = high.bitor(low);
            high = lhs[i].wrapping_shl(shl_shift);
        }

        let mut ret = Self::ZERO;
        ret.low.limbs.copy_from_slice(&limbs[0..LIMBS]);
        ret.high.limbs.copy_from_slice(&limbs[LIMBS..new_limbs]);

        ret
    }

    #[inline(always)]
    fn rem(&self, rhs: &Uint<LIMBS>) -> Uint<LIMBS> {
        let lz = rhs.leading_zeros();
        let mut rem = *self;
        let mut rhs = Self {
            low: Uint::ZERO,
            high: rhs.wrapping_shl(lz as u32),
        };

        let mut bd = lz + Uint::<LIMBS>::BITS;

        loop {
            let (r, borrow) = rem.sbb(&rhs, Limb::ZERO);
            if borrow.is_zero() {
                rem = r;
            }
            if bd == 0 {
                break;
            }
            bd -= 1;
            rhs = rhs.shr1();
        }
        rem.low
    }
}

impl<const LIMBS: usize> From<(Uint<LIMBS>, Uint<LIMBS>)> for Wide<LIMBS> {
    fn from(v: (Uint<LIMBS>, Uint<LIMBS>)) -> Self {
        Self {
            low: v.0,
            high: v.1,
        }
    }
}

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub(crate) fn to_wide<const WIDE_LIMBS: usize>(self) -> Wide<WIDE_LIMBS> {
        assert_eq!(LIMBS / 2, WIDE_LIMBS);
        let mut r = Wide::ZERO;
        r.low.limbs.copy_from_slice(&self.limbs[0..WIDE_LIMBS]);
        r.high.limbs.copy_from_slice(&self.limbs[WIDE_LIMBS..LIMBS]);
        r
    }
}

#[cfg(test)]
mod test {
    use std::cmp::{max, min};

    use rand::{thread_rng, Rng};

    use crate::num::uint::{U128, U64};
    use crate::num::wide::Wide;

    #[test]
    fn test_sub() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u128 = rng.gen();

            let a = max(a, b);
            let b = min(a, b);
            let c = a - b;

            const LOW_MASK: u128 = u64::MAX as u128;
            let a_high: u64 = (a >> 64) as u64;
            let b_high: u64 = (b >> 64) as u64;
            let c_high: u64 = (c >> 64) as u64;
            let a_low: u64 = (a & LOW_MASK) as u64;
            let b_low: u64 = (b & LOW_MASK) as u64;
            let c_low: u64 = (c & LOW_MASK) as u64;

            assert_eq!(((a_high as u128) << 64) + a_low as u128, a);
            assert_eq!(((b_high as u128) << 64) + b_low as u128, b);

            let w_a = Wide::from((U64::from_u64(a_low), U64::from_u64(a_high)));
            let w_b = Wide::from((U64::from_u64(b_low), U64::from_u64(b_high)));
            let r = w_a.sub(&w_b);

            assert_eq!(U64::from_u64(c_low), r.low);
            assert_eq!(U64::from_u64(c_high), r.high);
        }
    }

    #[test]
    fn test_add() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u128 = rng.gen();

            let (c, overflow) = a.overflowing_add(b);
            if overflow {
                continue;
            }

            const LOW_MASK: u128 = u64::MAX as u128;
            let a_high: u64 = (a >> 64) as u64;
            let b_high: u64 = (b >> 64) as u64;
            let c_high: u64 = (c >> 64) as u64;
            let a_low: u64 = (a & LOW_MASK) as u64;
            let b_low: u64 = (b & LOW_MASK) as u64;
            let c_low: u64 = (c & LOW_MASK) as u64;

            assert_eq!(((a_high as u128) << 64) + a_low as u128, a);
            assert_eq!(((b_high as u128) << 64) + b_low as u128, b);

            let w_a = Wide::from((U64::from_u64(a_low), U64::from_u64(a_high)));
            let w_b = Wide::from((U64::from_u64(b_low), U64::from_u64(b_high)));
            let r = w_a.add(&w_b);

            assert_eq!(U64::from_u64(c_low), r.low);
            assert_eq!(U64::from_u64(c_high), r.high);
        }
    }

    #[test]
    fn test_to_wide() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();

            const LOW_MASK: u128 = u64::MAX as u128;
            let a_high: u64 = (a >> 64) as u64;
            let a_low: u64 = (a & LOW_MASK) as u64;

            let u = U128::from_u128(a);
            let w = u.to_wide();
            assert_eq!(U64::from_u64(a_high), w.high);
            assert_eq!(U64::from_u64(a_low), w.low);
        }
    }

    #[test]
    fn test_rem() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u64 = rng.gen();

            let ba = U128::from_u128(a);
            let bb = U64::from_u64(b);

            let w = ba.to_wide();
            let r = w.rem(&bb);
            assert_eq!(
                U64::from_u64((a % b as u128) as u64),
                r,
                "a: {}, b: {}",
                a,
                b
            );
        }
    }
}
