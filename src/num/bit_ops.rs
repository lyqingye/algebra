use crate::num::limb::Limb;
use crate::num::uint::Uint;
use std::ops::{BitOr, Shl, Shr};

impl<const LIMBS: usize> Uint<LIMBS> {
    pub fn leading_zeros(&self) -> usize {
        let mut count = 0usize;

        for i in (0..LIMBS).rev() {
            let l = self.limbs[i];
            if l == Limb::ZERO {
                count += Limb::BITS;
            } else {
                count += l.0.leading_zeros() as usize;
                break;
            }
        }
        count
    }

    pub fn bits(&self) -> usize {
        Self::BITS - self.leading_zeros()
    }

    pub(crate) fn bitor(&self, rhs: &Self) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = self.limbs[i].bitor(rhs.limbs[i]);
            i += 1;
        }

        Self { limbs }
    }

    #[inline(always)]
    pub(crate) fn wrapping_shl(&self, rhs: u32) -> Self {
        let shift_bit = rhs as usize;

        if shift_bit == 0 {
            return *self;
        }

        let mut limbs = [Limb::ZERO; LIMBS];

        let shift_num = shift_bit / Limb::BITS;
        let shl_shift = shift_bit % Limb::BITS;
        let shr_shift = Limb::BITS - shl_shift;

        let mut low = Limb::ZERO;
        for i in 0..(LIMBS - shift_num) {
            let high = self.limbs[i].wrapping_shl(shl_shift as u32);
            limbs[i + shift_num] = high.bitor(low);
            low = self.limbs[i].wrapping_shr(shr_shift as u32);
        }

        Self { limbs }
    }

    #[inline(always)]
    pub(crate) fn overflowing_shl(&self, rhs: u32) -> (Self, bool) {
        let r = self.wrapping_shl(rhs);
        (r, r.wrapping_shr(rhs) != *self)
    }

    #[inline(always)]
    pub(crate) fn wrapping_shr(&self, rhs: u32) -> Self {
        let shift_bit = rhs as usize;

        if shift_bit == 0 {
            return *self;
        }

        let mut limbs = [Limb::ZERO; LIMBS];

        let shift_num = shift_bit / Limb::BITS;
        let shr_shift = (shift_bit % Limb::BITS) as u32;
        let shl_shift = Limb::BITS as u32 - shr_shift;

        let mut high = Limb::ZERO;
        for i in (shift_num..LIMBS).rev() {
            let low = self.limbs[i].wrapping_shr(shr_shift);
            limbs[i - shift_num] = high.bitor(low);
            high = self.limbs[i].wrapping_shl(shl_shift);
        }

        Self { limbs }
    }

    #[inline(always)]
    pub(crate) fn overflowing_shr(&self, rhs: u32) -> (Self, bool) {
        let r = self.wrapping_shr(rhs);
        (r, *self != r.wrapping_shl(rhs))
    }
}
impl<const LIMBS: usize> Shr<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;
    fn shr(self, rhs: &Uint<LIMBS>) -> Self::Output {
        self.wrapping_shr(rhs.limbs[0].0 as u32)
    }
}

impl<const LIMBS: usize> Shl<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;
    fn shl(self, rhs: &Uint<LIMBS>) -> Self::Output {
        self.wrapping_shl(rhs.limbs[0].0 as u32)
    }
}

#[cfg(test)]
mod test {
    use crate::num::uint::U128;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_shr() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let random: u128 = rng.gen();
            let shift = rng.gen_range(0..64);

            let uint = U128::from_u128(random);
            let expect = U128::from_u128(random >> shift);
            let actual = uint >> &U128::from_u64(shift);

            assert_eq!(
                expect,
                actual,
                "shift: {} expect: {}, actual: {}",
                shift,
                expect.to_binary_string(),
                actual.to_binary_string()
            );
        }
    }

    #[test]
    fn test_shl() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let random: u128 = rng.gen();
            let shift = rng.gen_range(0..64);

            let uint = U128::from_u128(random);
            let expect = U128::from_u128(random << shift);
            let actual = uint << &U128::from_u64(shift);

            assert_eq!(
                expect,
                actual,
                "value: {} shift: {} expect: {}, actual: {}",
                random,
                shift,
                expect.to_binary_string(),
                actual.to_binary_string()
            );
        }
    }
}
