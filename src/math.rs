pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(1, 100), 1);
        assert_eq!(gcd(100, 1), 1);
        assert_eq!(gcd(6, 3), 3);
        assert_eq!(gcd(21, 6), 3);
        assert_eq!(gcd(48, 56), 8);
    }
}
