pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n
    }
    n
}

pub fn number_length(x: u64) -> u64 {
    let mut xc = x;
    let mut res = 0;
    while xc > 0 {
        res += 1;
        xc /= 10;
    }
    res
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

    #[test]
    fn test_number_length() {
        assert_eq!(number_length(100), 3);
        assert_eq!(number_length(999), 3);
        assert_eq!(number_length(1000), 4);
        assert_eq!(number_length(9999), 4);
    }
}
