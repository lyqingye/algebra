use crate::inverse::mod_inv;

pub struct Montgomery {
    n: u64,
    r: u64,
    r2: u64,
    k: u64,
    neg_inv_n: u64,
}

fn lo64(a: u64) -> u64 {
    a & ((1<<32)-1)
}

/// Returns the most significant 64 bits of a
fn hi64(a: u64) -> u64 {
    a >> 32
}

fn add64(a0: u64, a1: u64, b0: u64, b1: u64) -> (u64, u64) {
    let (r0, overflow) = a0.overflowing_add(b0);
    let r1 = a1.wrapping_add(b1).wrapping_add(overflow as u64);
    (r0, r1)
}

fn mul64(a: u64, b: u64) -> (u64, u64) {
    // Split a and b into hi and lo 64-bit parts
    // a*b = (a1<<64 + a0)*(b1<<64 + b0)
    // = (a1*b1)<<128 + (a1*b0 + b1*a0)<<64 + a0*b0
    let (a0, a1) = (lo64(a), hi64(a));
    let (b0, b1) = (lo64(b), hi64(b));
    let (x, y) = (a1*b0, b1*a0);

    let (r0, r1) = (a0*b0, a1*b1);
    let (r0, r1) = add64(r0, r1, lo64(x)<<32, hi64(x));
    let (r0, r1) = add64(r0, r1, lo64(y)<<32, hi64(y));
    (r0, r1)
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
    use crate::montgomery::{Montgomery, mul64};

    #[test]
    fn test_mod_mul() {
        let mont = Montgomery::init(123456789);
        assert_eq!(
            (23456789u128 * 12345678u128 % 123456789u128) as u64,
            mont.mod_mul(23456789, 12345678)
        );
    }

    #[test]
    fn test_mul_u128() {
        let x: u64 = 12345678901234567890;
        let y: u64 = 9876543210987654321;

        let result = mul64(x, y);

        println!("High: {}", result.0);
        println!("Low: {}", result.1);
        println!("{}", ((result.1 as u128) << 64) + result.0 as u128);
        println!("{}", (x as u128) * y as u128);
    }
}
