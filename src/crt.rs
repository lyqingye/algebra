use crate::inverse::modular_inverse;

fn chinese_remainder_theorem(a: Vec<i64>, m: Vec<i64>) -> Option<i64> {
    let prod: i64 = m.iter().product();
    let mut x = 0;

    for (a_i, m_i) in a.iter().zip(m.iter()) {
        let p = prod / m_i;

        x += a_i * modular_inverse(p, *m_i)? * p;
    }
    Some(x % prod)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crt() {
        assert_eq!(
            23,
            chinese_remainder_theorem([2, 3, 2].to_vec(), [3, 5, 7].to_vec()).unwrap()
        );
    }
}
