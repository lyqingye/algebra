use crate::ext_euc::ext_euc;
use crate::inverse::modular_inverse;

fn solve_linear_congruence(a: i64, b: i64, n: i64) -> Option<Vec<i64>> {
    let (d, _x, _y) = ext_euc(a, n);
    if b % d == 0 {
        let a_prime = a / d;
        let b_prime = b / d;
        let n_prime = n / d;
        let inv = modular_inverse(a_prime, n_prime)?;
        let x_0 = b_prime * inv % n_prime;

        let mut results = Vec::with_capacity(d as usize);
        for k in 0..d {
            let x = (k * n_prime + x_0) % n;
            results.push(x);
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
    fn test_solve_liner_congruence() {
        let result = solve_linear_congruence(214124, 4124, 344).unwrap();
        assert_eq!(11, result[0]);
        assert_eq!(97, result[1]);
        assert_eq!(183, result[2]);
        assert_eq!(269, result[3]);
    }
}
