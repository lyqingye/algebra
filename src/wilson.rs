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
