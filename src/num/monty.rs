use std::ops::Rem;

use crate::num::uint::Uint;
use crate::num::wide::Wide;

#[derive(Clone, Copy)]
pub struct MontyForm<const LIMBS: usize> {
    n: Uint<LIMBS>,
    r: Uint<LIMBS>,
    r2: Uint<LIMBS>,
    k: u32,
    neg_inv_n: Uint<LIMBS>,
}

impl<const LIMBS: usize> MontyForm<LIMBS> {
    pub fn init(n: &Uint<LIMBS>) -> Option<Self> {
        let n = *n;
        let k = Uint::<LIMBS>::BITS as u32;
        let r = Self::r(&n);
        let r2 = r.split_mul(&r).rem(&n);
        let inv_n = n.mod_inv(&r)?;
        let neg_inv_n = r - &inv_n;
        Some(Self {
            n,
            r,
            r2,
            k,
            neg_inv_n,
        })
    }

    fn r(n: &Uint<LIMBS>) -> Uint<LIMBS> {
        Uint::pow2k_mod(Uint::<LIMBS>::BITS as u32, n)
    }

    pub fn reduction(&self, t: &Uint<LIMBS>) -> Uint<LIMBS> {
        let m = t.rem(self.r).split_mul(&self.neg_inv_n).rem(&self.r);
        let ret = m
            .split_mul(&self.n)
            .add(&Wide::from((*t, Uint::ZERO)))
            .div(&self.r)
            .0
            .low;
        if ret >= self.n {
            ret - &self.n
        } else {
            ret
        }
    }

    pub fn mul_mod(&self, a: &Uint<LIMBS>, b: &Uint<LIMBS>) -> Uint<LIMBS> {
        assert!(a < &self.n);
        assert!(b < &self.n);

        let ar = self.reduction(&(a.split_mul(&self.r2).rem(&self.n)));
        let br = self.reduction(&(b.split_mul(&self.r2).rem(&self.n)));
        let abr2 = ar.split_mul(&br).rem(&self.n);
        let abr = self.reduction(&abr2);
        self.reduction(&abr)
    }
}

#[cfg(test)]
mod test {
    use crate::num::monty::MontyForm;
    use crate::num::uint::U64;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_mod_mul() {
        let a = U64::from_u64(23456789);
        let b = U64::from_u64(12345678);
        let m = U64::from(123456789u64);
        let mont = MontyForm::init(&m).unwrap();
        let r = mont.mul_mod(&a, &b);
        assert_eq!(a.mul_mod(&b, &m), r);
    }

    #[test]
    fn test_mod_mul_rand() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a = U64::from_u64(rng.gen());
            let b = U64::from_u64(rng.gen());
            let m = U64::from_u64(rng.gen_range(1..=u64::MAX));
            if m <= a || m <= b {
                continue;
            }
            let mont = MontyForm::init(&m);
            if mont.is_none() {
                continue;
            }
            let r = mont.unwrap().mul_mod(&a, &b);
            assert_eq!(a.mul_mod(&b, &m), r);
        }
    }
}
