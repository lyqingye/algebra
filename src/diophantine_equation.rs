use crate::ext_euc::ext_euc;

fn solve_diophantine_equation(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    let (d, x_0, y_0) = ext_euc(a, b);

    if c % d == 0 {
        let k = c / d;
        Some((x_0 * k, y_0 * k))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_diophantine_equation() {
        assert_eq!(Some((-5, 5)), solve_diophantine_equation(3, 4, 5));
        assert_eq!(
            Some((168498, -2048886)),
            solve_diophantine_equation(51241, 4214, 414)
        );
    }
}
