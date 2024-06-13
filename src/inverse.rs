use crate::ext_euc::ext_euc;

fn modular_inverse(a: i64, m: u64) -> Option<u64> {
    let (d, x, _y) = ext_euc(a, m as i64);
    if d == 1 {
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
    fn test_modular_inverse() {
        assert_eq!(Some(5), modular_inverse(3, 7));
        assert_eq!(Some(7), modular_inverse(3, 10));
        assert_eq!(Some(15), modular_inverse(7, 26));

        assert_eq!(None, modular_inverse(2, 6));
        assert_eq!(None, modular_inverse(6, 12));
        assert_eq!(None, modular_inverse(5, 15));
        assert_eq!(None, modular_inverse(7, 21));
    }
}
