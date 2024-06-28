use crate::inverse::mod_inv;

pub struct Montgomery {
    n: u64,
    r: u64,
    r2: u64,
    k: u64,
    neg_inv_n: u64,
}

impl Montgomery {
    pub fn init(n: u64) -> Self {
        let k = 63;
        let r = 9223372036854775808;
        assert!(n < r);
        let inv_n = mod_inv(n, r).unwrap();
        let neg_inv_n = r - inv_n;
        let r2 = (r as u128 * r as u128 % n as u128) as u64;
        Self {
            n,
            r,
            r2,
            k,
            neg_inv_n,
        }
    }

    pub fn reduction(&self, t: u64) -> u64 {
        let m = ((t % self.r) as u128 * self.neg_inv_n as u128 % self.r as u128) as u64;
        let ret = ((m as u128 * self.n as u128 + t as u128) >> self.k) as u64;
        if ret > self.n {
            ret - self.n
        } else {
            ret
        }
    }
    pub fn mod_mul(&self, a: u64, b: u64) -> u64 {
        assert!(a < self.n);
        assert!(b < self.n);
        let ar = self.reduction(((a as u128 * self.r2 as u128) % self.n as u128) as u64);
        let br = self.reduction(((b as u128 * self.r2 as u128) % self.n as u128) as u64);
        let abr2 = ar as u128 * br as u128;
        let abr = self.reduction((abr2 % self.n as u128) as u64);
        self.reduction(abr)
    }
}

#[cfg(test)]
mod test {
    use crate::montgomery::Montgomery;

    #[test]
    fn test_mod_mul() {
        let mont = Montgomery::init(123456789);
        assert_eq!(
            (23456789u128 * 12345678u128 % 123456789u128) as u64,
            mont.mod_mul(23456789, 12345678)
        );
    }
}
