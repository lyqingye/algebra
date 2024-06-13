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

#[cfg(test)]
mod test {
    use super::*;

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
}
