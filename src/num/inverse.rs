use crate::num::uint::Uint;
use std::ops::Shr;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub fn mod_inv(&self, modulus: &Self) -> Option<Self> {
        // Based on the inverse pseudocode listed here:
        // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers

        assert!(
            !modulus.is_zero(),
            "attempt to calculate with zero modulus!"
        );
        if modulus == &Self::ONE {
            return Some(Self::ZERO);
        }

        let mut r0; // = modulus.clone();
        let mut r1 = *self % modulus;
        let mut t0; // = Self::zero();
        let mut t1; // = Self::one();

        // Lift and simplify the first iteration to avoid some initial allocations.
        if r1.is_zero() {
            return None;
        } else if r1 == Self::ONE {
            return Some(r1);
        } else {
            let (q, r2) = modulus.div_rem(&r1);
            if r2.is_zero() {
                return None;
            }
            r0 = r1;
            r1 = r2;
            t0 = Self::ONE;
            t1 = *modulus - &q;
        }

        while !r1.is_zero() {
            let (q, r2) = r0.div_rem(&r1);
            r0 = r1;
            r1 = r2;

            // let qt1 = q * &t1 % modulus;
            let qt1 = q.mul_mod(&t1, modulus);
            let t2 = if t0 < qt1 {
                t0 + &(*modulus - &qt1)
            } else {
                t0 - &qt1
            };
            t0 = t1;
            t1 = t2;
        }

        if r0 == Self::ONE {
            Some(t0)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn mod_inv_2k_dusse_kaliski(&self, k: u32) -> Option<Self> {
        assert!(k >= Self::BITS as u32);
        if self.is_even() {
            return None;
        }
        let two = Self::from(2u64);
        let mut x = Self::ONE;

        // 代表二进制 11, i 从1开始，所以 2^(i+1) - 1 = 3
        let mut mask = Self::from(3u64);

        // 递推式:
        // x_{k+1} = x_{k}(2-ax{k}) \pmod {2^{k+1}}
        for _ in 1..k {
            let a_x = self.wrapping_mul(&x).bitand(&mask);
            let two_minus_a_x = two.wrapping_sub(&a_x).bitand(&mask);
            x = x.wrapping_mul(&two_minus_a_x).bitand(&mask);

            mask = mask.wrapping_shl1();
            mask = mask.bitor(&Self::ONE);
        }

        Some(x)
    }

    #[inline(always)]
    pub fn mod_inv_2k(&self, k: u32) -> Option<Self> {
        assert!(k >= Self::BITS as u32);
        if self.is_even() {
            return None;
        }

        let mut x = Self::ZERO;
        let mut b = Self::ONE;

        for i in 0..k {
            let x_i = b.limbs[0].0 & 1;

            if x_i == 1 {
                b = b.wrapping_sub(self).wrapping_shr1();
            } else {
                b = b.shr(&Self::ONE);
            }

            let set_bit_mask = Uint::from_u64(x_i).wrapping_shl(i);
            x = x.bitor(&set_bit_mask);
        }

        Some(x)
    }
}

#[cfg(test)]

mod test {
    use crate::inverse::mod_inverse_2k;
    use crate::num::uint::{U128, U64};
    use rand::{thread_rng, Rng};

    #[test]
    fn test_mod_inverse() {
        assert_eq!(
            Some(U128::from_u128(2065)),
            U128::from_u128(32399933).mod_inv(&U128::from_u128(3233))
        );
        assert_eq!(
            Some(U128::from_u128(72527)),
            U128::from_u128(323213123).mod_inv(&U128::from_u128(323233))
        );
        assert_eq!(None, U128::from_u128(4).mod_inv(&U128::from_u128(2)));
    }

    #[test]
    fn test_mod_inv_2k() {
        assert_eq!(
            Some(U128::from_u128(8978285766154701517249939112421813611)),
            U128::from_u128(323213123u128).mod_inv_2k(128)
        );
    }

    #[test]
    fn test_mod_inv_2k_rand() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u64 = rng.gen();
            let k = 64;
            let expect = mod_inverse_2k(a, k).map(U64::from_u64);
            let actual = U64::from_u64(a).mod_inv_2k(k);
            let actual2 = U64::from_u64(a).mod_inv_2k_dusse_kaliski(k);
            assert_eq!(expect, actual);
            assert_eq!(expect, actual2)
        }
    }
}
