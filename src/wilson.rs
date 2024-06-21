fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let mut factorial: u64 = 1;

    // NOTE: low performance, but it can avoid overflow
    for i in 1..=(n - 1) {
        factorial = (factorial * i) % n;
    }

    (factorial + 1) % n == 0
}

pub fn is_prime2(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    let sqrt_n = (n as f64).sqrt() as u64;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
    }
}
