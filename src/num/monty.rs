use std::ops::Rem;

use crate::num::limb::Limb;
use crate::num::uint::Uint;
use crate::num::wide::Wide;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MontyParams<const LIMBS: usize> {
    n: Uint<LIMBS>,
    r: Uint<LIMBS>,
    r2: Uint<LIMBS>,
    neg_inv_n: Uint<LIMBS>,
}

impl<const LIMBS: usize> MontyParams<LIMBS> {
    #[inline(always)]
    pub fn init(n: &Uint<LIMBS>) -> Option<Self> {
        let n = *n;

        // 2^k mod p = 2^k - 1 + 1 mod p = 2^k - 1 mod p + 1 mod p = Uint::MAX mod p + 1
        let r = Uint::MAX.rem(&n).wrapping_add(&Uint::ONE);
        let r2 = r.split_mul(&r).rem(&n);
        let inv_n = n.mod_inv_2k(Uint::<LIMBS>::BITS as u32)?;
        let neg_inv_n = Wide::from((Uint::ZERO, Uint::ONE))
            .sub(&Wide::from((inv_n, Uint::ZERO)))
            .low;
        Some(Self {
            n,
            r,
            r2,
            neg_inv_n,
        })
    }

    #[inline(always)]
    pub fn reduction(&self, t: &Uint<LIMBS>) -> Uint<LIMBS> {
        let m = t.split_mul(&self.neg_inv_n).low;
        let ret = m.split_mul(&self.n).add(&Wide::from((*t, Uint::ZERO))).high;
        if ret >= self.n {
            ret - &self.n
        } else {
            ret
        }
    }

    #[inline(always)]
    pub fn reduction_wide(&self, t: &Wide<LIMBS>) -> Uint<LIMBS> {
        // m = (t * neg_inv_n) % r
        let tm = t.mul(&Wide::from((self.neg_inv_n, Uint::ZERO)));
        let mut m = Uint::ZERO;
        m.limbs.copy_from_slice(&tm[0..LIMBS]);

        // mn + t
        let mn = m.split_mul(&self.n).to_limbs();
        let mut mn_t = vec![Limb::ZERO; LIMBS * 4];
        let t_rhs = t.to_limbs();
        let mut carry = Limb::ZERO;
        for i in 0..LIMBS * 2 {
            let (w, c) = mn[i].adc(t_rhs[i], carry);
            mn_t[i] = w;
            carry = c;
        }
        mn_t[LIMBS * 2] = carry;

        // (mn + t) / r
        let mut ret = Wide::ZERO;
        ret.low.limbs.copy_from_slice(&mn_t[LIMBS..2 * LIMBS]);
        ret.high.limbs.copy_from_slice(&mn_t[LIMBS * 2..3 * LIMBS]);

        let n_wide = Wide::from((self.n, Uint::ZERO));
        if ret >= n_wide {
            ret.sub(&n_wide).low
        } else {
            ret.low
        }
    }

    /// 映射到蒙哥马利空间
    #[inline(always)]
    pub fn to_monty_form(self, t: &Uint<LIMBS>) -> MontyForm<LIMBS> {
        let form = self.reduction(&(t.split_mul(&self.r2).rem(&self.n)));
        MontyForm { form, params: self }
    }

    #[inline(always)]
    pub fn to_monty_form_wide(self, t: &Uint<LIMBS>) -> MontyForm<LIMBS> {
        let form = self.reduction_wide(&(t.split_mul(&self.r2)));
        MontyForm { form, params: self }
    }

    #[inline(always)]
    pub fn simply_to_monty_form(self, t: &Uint<LIMBS>) -> MontyForm<LIMBS> {
        MontyForm {
            form: t.split_mul(&self.r).rem(&self.n),
            params: self,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MontyForm<const LIMBS: usize> {
    form: Uint<LIMBS>,
    params: MontyParams<LIMBS>,
}

impl<const LIMBS: usize> MontyForm<LIMBS> {
    #[inline(always)]
    pub fn mul(&self, rhs: &Self) -> Self {
        Self {
            form: self.params.reduction_wide(&self.form.split_mul(&rhs.form)),
            params: self.params,
        }
    }

    /// 从蒙哥马利空间转换为标准形式
    #[inline(always)]
    pub fn normalize(&self) -> Uint<LIMBS> {
        self.params.reduction(&self.form)
    }
}

#[cfg(test)]
mod test {
    use std::u128;

    use rand::{thread_rng, Rng};

    use crate::num::monty::MontyParams;
    use crate::num::uint::U128;

    #[test]
    fn test_mod_mul() {
        let a = U128::from_u128(230679353788795331459744549142118481455);
        let b = U128::from_u128(146263473042228956998536595460379662786);
        let m = U128::from_u128(287215270712012985982119861826231487661);
        let params = MontyParams::init(&m).unwrap();
        let ma = params.to_monty_form(&a);
        let mb = params.to_monty_form(&b);
        let ma_mod_mb = ma.mul(&mb);
        let r = ma_mod_mb.normalize();

        println!("expect: {} actual: {}", a.mul_mod(&b, &m), r);
        assert_eq!(a.mul_mod(&b, &m), r);
    }

    #[test]
    fn test_mod_mul_rand() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a = U128::from_u128(rng.gen());
            let b = U128::from_u128(rng.gen());
            let m = U128::from_u128(rng.gen_range(1u128..=u128::MAX));
            if m <= a || m <= b {
                continue;
            }
            let params = MontyParams::init(&m);
            if params.is_none() {
                continue;
            }
            let ma = params.unwrap().to_monty_form_wide(&a);
            let mb = params.unwrap().to_monty_form_wide(&b);
            let r = ma.mul(&mb).normalize();
            assert_eq!(a.mul_mod(&b, &m), r, "a: {} b: {} m: {}", a, b, m);
        }
    }

    #[test]
    fn test_to_monty_form() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a = U128::from_u128(rng.gen());
            let b = U128::from_u128(rng.gen());
            let m = U128::from_u128(rng.gen_range(1u128..=u128::MAX));
            if m <= a || m <= b {
                continue;
            }
            let params = MontyParams::init(&m);
            if params.is_none() {
                continue;
            }
            let ma = params.unwrap().to_monty_form(&a);
            let mb = params.unwrap().to_monty_form(&b);

            assert_eq!(ma, params.unwrap().simply_to_monty_form(&a));
            assert_eq!(mb, params.unwrap().simply_to_monty_form(&b));

            assert_eq!(
                ma,
                params.unwrap().to_monty_form_wide(&a),
                "a: {} b: {} m:{}",
                a,
                b,
                m
            );
            assert_eq!(mb, params.unwrap().to_monty_form_wide(&b));
        }
    }

    #[test]
    fn test_to_monty_form2() {
        let a = U128::from_u128(294124311259551781831685531873756646517);
        let b = U128::from_u128(245879691615435039602073817273427569502);
        let m = U128::from_u128(337689454336161297985637674404251423519);
        let params = MontyParams::init(&m);
        let ma = params.unwrap().to_monty_form(&a);
        let mb = params.unwrap().to_monty_form(&b);

        assert_eq!(ma, params.unwrap().simply_to_monty_form(&a));
        assert_eq!(mb, params.unwrap().simply_to_monty_form(&b));

        assert_eq!(
            ma,
            params.unwrap().to_monty_form_wide(&a),
            "a: {} b: {} m:{}",
            a,
            b,
            m
        );
        assert_eq!(
            mb,
            params.unwrap().to_monty_form_wide(&b),
            "a: {} b: {} m:{}",
            a,
            b,
            m
        );
    }
}
