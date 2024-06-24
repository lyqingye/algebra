pub fn ext_euc_rec(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, x_0, y_0) = ext_euc_rec(b, a % b);
    let x_1 = y_0;
    let y_1 = x_0 - (a / b) * y_0;
    (d, x_1, y_1)
}

pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        // q = a / b
        let q = old_r / r;

        // r = a % b
        let new_r = old_r - q * r;

        let new_s = old_s - q * s;
        let new_t = old_t - q * t;

        // new_a = b
        // new_b = a % b
        old_r = r;
        r = new_r;

        old_s = s;
        s = new_s;

        old_t = t;
        t = new_t;
    }

    (old_r, old_s, old_t)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gcd::gcd;

    #[test]
    fn test_ext_gcd_rec() {
        let a = 240;
        let b = 46;
        let (d, x, y) = ext_euc_rec(a, b);
        assert_eq!(gcd(a, b), d);
        assert_eq!(d, a * x + b * y);
    }

    #[test]
    fn test_ext_gcd() {
        let a = 240;
        let b = 46;
        let (d, x, y) = ext_gcd(a, b);
        assert_eq!(gcd(a, b), d);
        assert_eq!(d, a * x + b * y);
    }
}
