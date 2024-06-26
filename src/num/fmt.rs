use crate::num::uint::Uint;
use std::fmt;

impl<const LIMBS: usize> Uint<LIMBS> {
    #[inline(always)]
    pub(crate) fn to_binary_string(self) -> String {
        let mut result = String::with_capacity(LIMBS);

        for i in (0..LIMBS).rev() {
            let limb = self.limbs[i];
            result.push_str(limb.to_binary_string(false).as_str())
        }

        result
    }
}

impl<const LIMBS: usize> fmt::Display for Uint<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let mut current = *self;

        while !current.is_zero() {
            let (next, remainder) = current.div_rem(&Self::TEN);
            result.push_str(remainder.limbs[0].0.to_string().as_str());
            current = next;
        }

        if result.is_empty() {
            result.push('0');
        }

        result.chars().rev().collect::<String>().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::num::uint::U128;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_to_string() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u128 = rng.gen();
            assert_eq!(a.to_string(), U128::from_u128(a).to_string());
        }
    }

    #[test]
    fn test_to_binary_string() {
        assert_eq!(
        "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001111",
        U128::from_u64(0b1111).to_binary_string()
        );
        assert_eq!(
        "00000000000000000000000000000000000000000000000000000000000000001111111111111111111111111111111111111111111111111111111111111111",
        U128::from_u64(u64::MAX).to_binary_string()
        );
    }
}
