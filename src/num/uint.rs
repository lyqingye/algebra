use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, BitOr, Div, Mul, Rem, Shl, Shr, Sub};

use crate::num::limb::Limb;

#[derive(Copy, Clone, Eq, Debug)]
pub struct Uint<const LIMBS: usize> {
    pub(crate) limbs: [Limb; LIMBS],
}

impl<const LIMBS: usize> Uint<LIMBS> {
    pub const ZERO: Self = Self::from_u64(0);
    pub const ONE: Self = Self::from_u64(1);
    pub const MAX: Self = Self {
        limbs: [Limb::MAX; LIMBS],
    };

    #[inline(always)]
    pub fn adc(&self, rhs: &Self, mut carry: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            let (w, c) = self.limbs[i].adc(rhs.limbs[i], carry);
            limbs[i] = w;
            carry = c;
            i += 1;
        }

        (Self { limbs }, carry)
    }

    #[inline(always)]
    pub fn sbb(&self, rhs: &Self, mut borrow: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            let (w, b) = self.limbs[i].sbb(rhs.limbs[i], borrow);
            limbs[i] = w;
            borrow = b;
            i += 1;
        }

        (Self { limbs }, borrow)
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&limb| limb == Limb::ZERO)
    }

    #[inline(always)]
    fn overflowing_mul(&self, rhs: &Self) -> (Self, bool) {
        let mut temp = vec![Limb::ZERO; LIMBS * 2];

        let mut carry = Limb::ZERO;
        for i in 0..LIMBS {
            for j in 0..LIMBS {
                let (ret, c) = temp[i + j].mac(self.limbs[i], rhs.limbs[j], carry);
                carry = c;
                temp[i + j] = ret;

                if (i + j) >= LIMBS && ((temp[i + j] != Limb::ZERO) || (carry != Limb::ZERO)) {
                    return (Self::ZERO, true);
                }
            }
            temp[i + LIMBS] = carry;
        }

        let mut result = [Limb::ZERO; LIMBS];
        result.copy_from_slice(&temp[0..LIMBS]);

        (Self { limbs: result }, false)
    }

    fn overflowing_shl(&self, shift: &Self) -> (Self, bool) {
        let shift_bit = shift.limbs[0].0 as usize;
        let mut limbs = [Limb::ZERO; LIMBS];

        let shift_num = shift_bit / Limb::BITS;
        let shl_shift: Limb = (shift_bit % Limb::BITS).into();
        let shr_shift: Limb = (Limb::BITS - shl_shift.0 as usize).into();

        let mut low = Limb::ZERO;
        for i in 0..(LIMBS - shift_num) {
            let high = self.limbs[i].shl(shl_shift);
            limbs[i + shift_num] = high.bitor(low);
            low = self.limbs[i].shr(shr_shift);
        }

        (Self { limbs }, Self { limbs }.shr(shift) != *self)
    }

    fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        assert_ne!(divisor, &Self::ZERO);

        let mut quotient = Self::ZERO;
        let mut d2 = *divisor;
        let mut pow = Self::ONE;
        loop {
            let (n, overflow) = d2.overflowing_shl(&Self::ONE);
            if overflow || (n > *self) {
                break;
            }
            d2 = n;
            pow = pow << &Self::ONE;
        }

        let mut remainder = *self;
        while remainder >= *divisor && d2 != Self::ZERO {
            let (r, borrow) = remainder.sbb(&d2, Limb::ZERO);
            if borrow == Limb::ZERO {
                remainder = r;
                quotient = quotient + &pow;
            }
            d2 = d2 >> &Self::ONE;
            pow = pow >> &Self::ONE;
        }

        (quotient, remainder)
    }
    const fn from_u64(n: u64) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }

    const fn from_u128(n: u128) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as u64;
        limbs[1].0 = (n >> Limb::BITS) as u64;
        Self { limbs }
    }

    fn leading_zeros(&self) -> usize {
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

    fn bits(&self) -> usize {
        (LIMBS * Limb::BITS) - self.leading_zeros()
    }

    pub fn bitor(&self, rhs: &Self) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = self.limbs[i].bitor(rhs.limbs[i]);
            i += 1;
        }

        Self { limbs }
    }
}

// Impl Ops

impl<const LIMBS: usize> Add<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let (r, s) = self.adc(rhs, Limb::ZERO);
        assert_eq!(s, Limb::ZERO, "attempted to add with overflow");
        r
    }
}

impl<const LIMBS: usize> Sub<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let (r, s) = self.sbb(rhs, Limb::ZERO);
        assert_eq!(s, Limb::ZERO, "attempted to subtract with overflow");
        r
    }
}

impl<const LIMBS: usize> Mul<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;
    fn mul(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let (product, overflow) = self.overflowing_mul(rhs);
        assert!(!overflow, "attempted to multiple with overflow");
        product
    }
}

impl<const LIMBS: usize> Div<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;
    fn div(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let (q, _r) = self.div_rem(rhs);
        q
    }
}

impl<const LIMBS: usize> Rem<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;

    fn rem(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let (_q, r) = self.div_rem(rhs);
        r
    }
}

// Impl Logic Ops
impl<const LIMBS: usize> Shr<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;
    fn shr(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let shift_bit = rhs.limbs[0].0 as usize;
        let mut limbs = [Limb::ZERO; LIMBS];
        let shift_num = shift_bit / Limb::BITS;
        let rem: Limb = (shift_bit % Limb::BITS).into();
        let mut index = 0;

        for i in shift_num..LIMBS - 1 {
            let high = self.limbs[i + 1] << (Limb::BITS - rem.0 as usize).into();
            let low = self.limbs[i].shr(rem);
            limbs[index] = high.bitor(low);
            index += 1;
        }

        limbs[index] = self.limbs[LIMBS - 1] >> rem;

        Self { limbs }
    }
}

impl<const LIMBS: usize> Shl<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;
    fn shl(self, rhs: &Uint<LIMBS>) -> Self::Output {
        let (r, _overflow) = self.overflowing_shl(rhs);
        r
    }
}

// Impl From

impl<const LIMBS: usize> From<u64> for Uint<LIMBS> {
    fn from(n: u64) -> Self {
        Self::from_u64(n)
    }
}

// Impl Cmp
impl<const LIMBS: usize> PartialEq for Uint<LIMBS> {
    fn eq(&self, other: &Self) -> bool {
        self.limbs.eq(&other.limbs)
    }
}

impl<const LIMBS: usize> Ord for Uint<LIMBS> {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..LIMBS).rev() {
            match self.limbs[i].0.cmp(&other.limbs[i].0) {
                Ordering::Less => {
                    return Ordering::Less;
                }
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => {}
            }
        }
        Ordering::Equal
    }
}

impl<const LIMBS: usize> PartialOrd for Uint<LIMBS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Impl Display

impl<const LIMBS: usize> fmt::Display for Uint<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let mut current = *self;

        while !current.is_zero() {
            let (next, remainder) = current.div_rem(&Self::from_u64(10));
            result.push_str(remainder.limbs[0].0.to_string().as_str());
            current = next;
        }

        if result.is_empty() {
            result.push('0');
        }

        result.chars().rev().collect::<String>().fmt(f)
    }
}

pub type U128 = Uint<2>;

#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn test_mul() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            let (c, overflow) = (a as u128).overflowing_mul(b as u128);
            if overflow {
                continue;
            }

            let a_1 = U128::from(a);
            let b_1 = U128::from(b);
            let c_1 = a_1 * &b_1;

            assert_eq!(c.to_string(), c_1.to_string())
        }
        let mut t = U128::from(2);
        for _ in 0..126 {
            t = t * &U128::from(2);
            println!("{:}", t);
        }
    }

    #[test]
    fn test_from_u128() {
        let mut rng = thread_rng();
        for _ in 0..10000 {
            let v: u128 = rng.gen();
            let v2 = U128::from_u128(v);
            assert_eq!(v2.to_string(), v.to_string());
        }
    }

    #[test]
    fn test_shr() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let v: u128 = rng.gen();
            let shift = rng.gen_range(0..64);
            let v2 = U128::from_u128(v);
            assert_eq!(
                (v2 >> &U128::from_u64(shift)).to_string(),
                (v >> shift).to_string(),
                "v: {}, shift: {}",
                v,
                shift
            );
        }
    }

    #[test]
    fn test_shl() {
        let mut rng = thread_rng();
        for _ in 0..10000 {
            let v: u128 = rng.gen();
            let shift = rng.gen_range(0..64);
            let v2 = U128::from_u128(v);
            assert_eq!(
                (v2 << &U128::from_u64(shift)).to_string(),
                (v << shift).to_string(),
                "v: {}, shift: {}",
                v,
                shift
            );
        }
    }
    #[test]
    fn test_div() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            let b: u64 = rng.gen();
            let ba = U128::from_u128(a);
            let bb = U128::from_u64(b);
            println!("a: {}, b: {}", a, b);
            assert_eq!(
                (ba / &bb).to_string(),
                (a / b as u128).to_string(),
                "a: {}, b: {}",
                a,
                b
            );
            println!("{:}", ba / &bb);
        }
    }

    #[test]
    fn test_div2() {
        let (q, r) = U128::from_u128(20).div_rem(&U128::from_u128(10));
        println!("{}", q);
        println!("{}", r);
    }

    #[test]
    fn test_to_string() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            assert_eq!(a.to_string(), U128::from_u128(a).to_string());
        }
    }
}
