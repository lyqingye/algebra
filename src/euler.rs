fn euler_phi(_n: u64) -> u64 {
    let mut n = _n;
    let mut result = n;
    let mut i = 2;

    // 质因数分解范围: [2,\sqrt{n}]
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            // result / i 为占比
            // result -= result / i 就是去掉后剩余的元素数量
            result -= result / i;
        }
        i += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euler_phi() {
        assert_eq!(4, euler_phi(10));
        assert_eq!(1296, euler_phi(2331));
    }
}
