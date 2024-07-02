use crate::num::uint::Uint;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    /// 2^k \mod n
    pub fn pow2k_mod(k: u32, n: &Self) -> Self {
        Uint::from(2u64).mod_exp(k, n)
    }

    #[inline(always)]
    /// self^{k} \mod n
    pub fn mod_exp(&self, exp: u32, n: &Self) -> Self {
        let mut result = Uint::ONE;
        let mut base = *self;
        let mut exp = exp;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul_mod(&base, n);
            }
            base = base.mul_mod(&base, n);
            exp /= 2;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::num::uint::U64;

    #[test]
    fn test_pow2k_mod() {
        let actual = U64::pow2k_mod(U64::BITS as u32, &U64::from_u64(3123123));
        assert_eq!(U64::from(294187u64), actual)
    }
}
