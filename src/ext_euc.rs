pub fn ext_euc_recursion(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, x_0, y_0) = ext_euc_recursion(b, a % b);
    let x_1 = y_0;
    let y_1 = x_0 - (a / b) * y_0;
    return (d, x_1, y_1);
}

pub fn ext_euc(_a: i64, _b: i64) -> (i64, i64, i64) {
    let mut x0 = 1;
    let mut y0 = 0;
    let mut x1 = 0;
    let mut y1 = 1;
    let mut a = _a;
    let mut b = _b;

    while b != 0 {
        let q = a / b;
        let r = a - q * b;
        let x = x0 - q * x1;
        let y = y0 - q * y1;

        a = b;
        b = r;
        x0 = x1;
        x1 = x;
        y0 = y1;
        y1 = y;
    }

    (a, x0, y0)
}

#[cfg(test)]
mod test {
    use crate::gcd::gcd;
    use super::*;

    #[test]
    fn test_ext_euc_recursion() {
        let a = 240;
        let b = 46;
        let (d, x, y) = ext_euc_recursion(a, b);
        assert_eq!(gcd(a,b),d);
        assert_eq!(d, a * x + b * y);
    }

    #[test]
    fn test_ext_euc() {
        let a = 240;
        let b = 46;
        let (d, x, y) = ext_euc(a, b);
        assert_eq!(gcd(a,b),d);
        assert_eq!(d, a * x + b * y);
    }
}