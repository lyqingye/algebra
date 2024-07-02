use std::ops::Rem;

use crate::num::limb::Limb;
use crate::num::uint::Uint;
use crate::num::wide::Wide;

#[derive(Clone, Copy)]
pub struct MontyParams<const LIMBS: usize> {
    n: Uint<LIMBS>,
    r2: Uint<LIMBS>,
    neg_inv_n: Uint<LIMBS>,
}

impl<const LIMBS: usize> MontyParams<LIMBS> {
    pub fn init(n: &Uint<LIMBS>) -> Option<Self> {
        let n = *n;
        let r = Uint::MAX.rem(&n).adc(&Uint::ONE, Limb::ZERO).0;
        let r2 = r.split_mul(&r).rem(&n);
        let inv_n = n.mod_inv_2k(Uint::<LIMBS>::BITS as u32)?;
        let neg_inv_n = Wide::from((Uint::ZERO, Uint::ONE))
            .sub(&Wide::from((inv_n, Uint::ZERO)))
            .low;
        Some(Self { n, r2, neg_inv_n })
    }

    pub fn reduction(&self, t: &Uint<LIMBS>) -> Uint<LIMBS> {
        let m = t.split_mul(&self.neg_inv_n).low;
        let ret = m.split_mul(&self.n).add(&Wide::from((*t, Uint::ZERO))).high;
        if ret >= self.n {
            ret - &self.n
        } else {
            ret
        }
    }

    pub fn to_monty_form(self, t: &Uint<LIMBS>) -> MontyForm<LIMBS> {
        let form = self.reduction(&(t.split_mul(&self.r2).rem(&self.n)));
        MontyForm { form, params: self }
    }
}

#[derive(Clone, Copy)]
pub struct MontyForm<const LIMBS: usize> {
    form: Uint<LIMBS>,
    params: MontyParams<LIMBS>,
}

impl<const LIMBS: usize> MontyForm<LIMBS> {
    pub fn mul_mod(&self, rhs: &Self) -> Self {
        let form = self.form.split_mul(&rhs.form).rem(&self.params.n);
        Self {
            form,
            params: self.params,
        }
    }

    pub fn recover(&self) -> Uint<LIMBS> {
        self.params.reduction(&self.params.reduction(&self.form))
    }
}

#[cfg(test)]
mod test {
    use crate::num::monty::MontyParams;
    use crate::num::uint::U64;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_mod_mul() {
        let a = U64::from_u64(23456789);
        let b = U64::from_u64(12345678);
        let m = U64::from(123456789u64);
        let params = MontyParams::init(&m).unwrap();
        let ma = params.to_monty_form(&a);
        let mb = params.to_monty_form(&b);
        let ma_mod_mb = ma.mul_mod(&mb);
        let r = ma_mod_mb.recover();

        // let r = mont.mul_mod(&a, &b);
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
            let params = MontyParams::init(&m);
            if params.is_none() {
                continue;
            }
            let ma = params.unwrap().to_monty_form(&a);
            let mb = params.unwrap().to_monty_form(&b);
            let r = ma.mul_mod(&mb).recover();
            assert_eq!(a.mul_mod(&b, &m), r);
        }
    }
}
