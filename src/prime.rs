fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut i = 2;

    // 质因数最大为 $\sqrt{n}$
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_prime_factors() {
        assert_eq!([2, 2, 3, 5].to_vec(), prime_factors(60));
        assert_eq!([2, 2, 3, 7, 491].to_vec(), prime_factors(41244));
    }
}
