use crate::num::uint::Uint;

impl<const LIMBS: usize> Uint<LIMBS> {
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

            let qt1 = q * &t1 % modulus;
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
}

#[cfg(test)]

mod test {
    use crate::num::uint::U128;

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
}
