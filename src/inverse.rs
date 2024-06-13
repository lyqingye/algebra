use crate::ext_euc::ext_euc;

fn inverse(a: i64, m: u64) -> Option<u64> {
    let (d, x, y) = ext_euc(a, m as i64);
    if (d == 1) {
        if x < 0 {
            Some((x + m as i64) as u64)
        } else {
            Some(x as u64)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inverse() {
        assert_eq!(Some(5), inverse(3, 7));
        assert_eq!(Some(7), inverse(3, 10));
        assert_eq!(Some(15), inverse(7, 26));

        assert_eq!(None, inverse(2, 6));
        assert_eq!(None, inverse(6, 12));
        assert_eq!(None, inverse(5, 15));
        assert_eq!(None, inverse(7, 21));
    }
}
