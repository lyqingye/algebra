use crate::num::limb::Limb;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Eq, Debug)]
pub struct Uint<const LIMBS: usize> {
    pub(crate) limbs: [Limb; LIMBS],
}

impl<const LIMBS: usize> Uint<LIMBS> {
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
    fn div_rem(&self, divisor: u64) -> (Self, u64) {
        let mut remainder = 0u64;
        let mut limbs = [Limb::ZERO; LIMBS];

        for i in (0..LIMBS).rev() {
            let (q, r) = (
                (self.limbs[i].0 as u128 + (remainder as u128) * (Limb::MAX.0 as u128 + 1))
                    / divisor as u128,
                (self.limbs[i].0 as u128 + (remainder as u128) * (Limb::MAX.0 as u128 + 1))
                    % divisor as u128,
            );
            limbs[i] = Limb(q as u64);
            remainder = r as u64;
        }

        (Self { limbs }, remainder)
    }

    fn schoolbook_mul(&self, rhs: &Self) -> Self {
        let mut temp = vec![Limb::ZERO; LIMBS * 2];

        let mut carry = Limb::ZERO;
        for i in 0..LIMBS {
            for j in 0..LIMBS {
                let (ret, c) = temp[i + j].mac(self.limbs[i], rhs.limbs[j], carry);
                carry = c;
                temp[i + j] = ret;

                if (i + j) >= LIMBS && ((temp[i + j] != Limb::ZERO) || (carry != Limb::ZERO)) {
                    panic!("overflow occurred in multiplication")
                }
            }
            temp[i + LIMBS] = carry;
        }

        let mut result = [Limb::ZERO; LIMBS];
        result.copy_from_slice(&temp[0..LIMBS]);

        Self { limbs: result }
    }
}

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
        self.schoolbook_mul(rhs)
    }
}

impl<const LIMBS: usize> From<u64> for Uint<LIMBS> {
    fn from(n: u64) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }
}

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

impl<const LIMBS: usize> fmt::Display for Uint<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let mut current = *self;

        while !current.is_zero() {
            let (next, remainder) = current.div_rem(10);
            result.push(char::from_digit(remainder as u32, 10).unwrap());
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
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_u2048_add_sub() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            let c = a as u128 * b as u128;

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
}
