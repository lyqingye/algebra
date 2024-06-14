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

fn find_all_solutions(a: i64, b: i64, c: i64, k: Vec<i64>) -> Option<Vec<(i64, i64)>> {
    let (d, x_0, y_0) = ext_euc(a, b);

    if c % d == 0 {
        let factor = c / d;
        let x = x_0 * factor;
        let y = y_0 * factor;

        let mut results = Vec::with_capacity(k.len());
        let m = a / d;
        let n = b / d;
        for k in k {
            results.push((x + k * n, y - k * m))
        }
        Some(results)
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

    #[test]
    fn test_find_all_solutions() {
        let result = find_all_solutions(3, 4, 5, (0..=5i64).collect()).unwrap();
        assert_eq!((-5, 5), result[0]);
        assert_eq!((-1, 2), result[1]);
        assert_eq!((3, -1), result[2]);
        assert_eq!((7, -4), result[3]);
        assert_eq!((11, -7), result[4]);
    }
}
