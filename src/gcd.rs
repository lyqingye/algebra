pub fn gcd_recursion(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd_recursion(y, x % y)
    }
}

pub fn gcd(_x: i64, _y: i64) -> i64 {
    let mut x = _x;
    let mut y = _y;
    loop {
        if y == 0 {
            return x;
        } else {
            let temp = x % y;
            x = y;
            y = temp;
        }
    }
}

pub fn gcd2(_x: u64, _y: u64) -> u64 {
    let mut x = _x;
    let mut y = _y;
    loop {
        if y == 0 {
            return x;
        } else {
            let temp = x % y;
            x = y;
            y = temp;
        }
    }
}

fn binary_gcd(mut u: u64, mut v: u64) -> u64 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let shift = (u | v).trailing_zeros();

    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u > v {
            u -= v;
            u >>= u.trailing_zeros();
        } else {
            v -= u;
            v >>= v.trailing_zeros();
        }
    }

    u << shift
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_gcd_recursion() {
        assert_eq!(5, gcd_recursion(15, 5));
        assert_eq!(4, gcd_recursion(148, 36));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(5, gcd(15, 5));
        assert_eq!(4, gcd(148, 36));
    }

    #[test]
    fn test_binary_gcd() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let a: u64 = rng.gen();
            let b: u64 = rng.gen();
            assert_eq!(gcd2(a, b), binary_gcd(a, b))
        }
    }
}
