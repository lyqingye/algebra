use crate::ext_euc::ext_gcd;

pub fn modular_inverse(a: i64, m: i64) -> Option<i64> {
    let (d, x, _y) = ext_gcd(a, m);
    if d == 1 {
        if x < 0 {
            Some(x + m)
        } else {
            Some(x)
        }
    } else {
        None
    }
}

pub fn mod_inv(a: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return Some(0);
    }
    let mut r0; // = modulus.clone();
    let mut r1 = a % m;
    let mut t0; // = Self::zero();
    let mut t1; // = Self::one();

    // Lift and simplify the first iteration to avoid some initial allocations.
    if r1 == 0 {
        return None;
    } else if r1 == 1 {
        return Some(r1);
    } else {
        let r2 = m % r1;
        let q = m / r1;
        if r2 == 0 {
            return None;
        }
        r0 = r1;
        r1 = r2;
        t0 = 1;
        t1 = m - q;
    }

    while r1 != 0 {
        let r2 = r0 % r1;
        let q = r0 / r1;
        r0 = r1;
        r1 = r2;

        let qt1 = (q as u128 * t1 as u128 % m as u128) as u64;
        let t2 = if t0 < qt1 { t0 + (m - qt1) } else { t0 - qt1 };
        t0 = t1;
        t1 = t2;
    }

    if r0 == 1 {
        Some(t0)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modular_inverse() {
        assert_eq!(Some(5), modular_inverse(3, 7));
        assert_eq!(Some(7), modular_inverse(3, 10));
        assert_eq!(Some(15), modular_inverse(7, 26));

        assert_eq!(None, modular_inverse(2, 6));
        assert_eq!(None, modular_inverse(6, 12));
        assert_eq!(None, modular_inverse(5, 15));
        assert_eq!(None, modular_inverse(7, 21));
    }

    #[test]
    fn test_modular_inverse2() {
        assert_eq!(Some(5), mod_inv(3, 7));
        assert_eq!(Some(7), mod_inv(3, 10));
        assert_eq!(Some(15), mod_inv(7, 26));

        assert_eq!(None, mod_inv(2, 6));
        assert_eq!(None, mod_inv(6, 12));
        assert_eq!(None, mod_inv(5, 15));
        assert_eq!(None, mod_inv(7, 21));
    }
}
