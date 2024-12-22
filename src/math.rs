use std::{
    iter::Sum,
    ops::{AddAssign, Mul},
};

/// Greatest common divisor of two numbers.
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n
    }
    n
}

/// Least common multiple of two numbers.
pub fn lcm(n: u64, m: u64) -> u64 {
    n * (m / gcd(n, m))
}

/// Get number of digits in an integer.
pub fn number_length(x: u64) -> u64 {
    let mut xc = x;
    let mut res = 0;
    while xc > 0 {
        res += 1;
        xc /= 10;
    }
    res
}

/// Solve normalized Bezout's identity.
/// Find x and y s.t. ax + by = 1,  assuming that a, b are co-prime.
pub fn bezout(a: i64, b: i64) -> Option<(i64, i64)> {
    if a < 0 {
        return bezout(-a, b).map(|(x, y)| (-x, y));
    }
    if b < 0 {
        return bezout(a, -b).map(|(x, y)| (x, -y));
    }

    let (mut ra, mut rb) = (a, b);
    let mut qs: Vec<i64> = Vec::with_capacity(20);

    // collect all quotients s.t. r[i] = q[i] * r[i+1] + r[i+2]
    while rb != 0 {
        qs.push(ra / rb);
        (ra, rb) = (rb, ra % rb);
    }

    if ra != 1 {
        // if gcd(a, b) > 1 we don't have a solution
        return None;
    }

    // Now,
    // r0 = q0 * r1 + r2                (r0, r1) = (a, b)
    // ...
    // r[n-2] = q[n-2] * r[n-1] + r[n]
    // r[n-1] = q[n-1] * r[n] + 1
    // r[n]   = q[n] * 1 + 0
    //
    // let c1, c2 be s.t. c1 r[n] + c2 r[n+1] = 1
    // start from last line where (r[n+1], r[n+2]) = (1, 0)
    let (mut c1, mut c2) = (1, 0);

    // using r[k] = r[k-2] - q[k-2] * r[k-1] we can assemble everything back
    // 1 = c1 r[k] + c2 r[k+1]
    //   = c1 r[k] + c2 (r[k-1] - q[k-1] * r[k])
    //   = c2 r[k-1] + (c1 - c2 * q[k-1]) r[k]
    while let Some(q) = qs.pop() {
        (c1, c2) = (c2, c1 - c2 * q);
    }

    Some((c1, c2))
}

/// Solve Diophantine equation ax + by = c.
/// Returns numbers (x0, y0, u, v) that define all solutions as
///   (x, y) = (x0, y0) + k (u, v) for k \in Z
pub fn diophantine(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)> {
    let gcd_ab = gcd(a.unsigned_abs(), b.unsigned_abs()) as i64;
    if c % gcd_ab != 0 {
        return None;
    }

    // solve normalized equation an x + bn x = 1
    let (mut x, mut y) = bezout(a / gcd_ab, b / gcd_ab)?;

    // multiply by cn to get solution to an x + bn y = cn, or equivalently to ax + by = c
    x *= c / gcd_ab;
    y *= c / gcd_ab;

    Some((x, y, b / gcd_ab, -a / gcd_ab))
}

/// Construct Chinese remainder theorem solution for two equations
/// Find x s.t.
///  x % n1 = a1
///  x % n2 = a2
pub fn crt2(n1: i64, a1: i64, n2: i64, a2: i64) -> Option<i64> {
    let (m1, m2) = bezout(n1, n2)?;
    let ret = (a1 % n1) * m2 * n2 + (a2 % n2) * m1 * n1;
    Some(ret.rem_euclid(n1 * n2))
}

/// Construct Chinese remainder theorem solution
/// Find x s.t.
///  x % nn[i] = aa[i],  i = 1..k
pub fn crt(nn: &[i64], aa: &[i64]) -> Option<i64> {
    let (mut n, mut x) = (nn[0], aa[0]);
    for (ni, ai) in nn[1..].iter().zip(aa[1..].iter()) {
        x = crt2(n, x, *ni, *ai)?;
        n *= ni;
    }
    Some(x)
}

/// Enumerate all nchoosek selections.
pub struct NChooseK {
    n: usize,
    indices: Vec<usize>,
    first: bool,
}

impl NChooseK {
    pub fn new(n: usize, k: usize) -> Self {
        let indices = (0..k).collect();
        assert!(k <= n);
        NChooseK {
            n,
            indices,
            first: true,
        }
    }
}

impl Iterator for NChooseK {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let k = self.indices.len();

        if self.first {
            self.first = false;
            return Some(self.indices.clone());
        }

        if k == 0 {
            return None;
        }

        // find highest index that can be incremented
        let mut i = k - 1;
        while i > 0 && self.indices[i] == self.n - k + i {
            i -= 1;
        }
        if i == 0 && self.indices[0] == self.n - k {
            return None; // we are done
        }

        // increment last index
        self.indices[i] += 1;

        // reset indices right of i
        for j in i + 1..k {
            self.indices[j] = self.indices[j - 1] + 1;
        }

        // can now return
        Some(self.indices.clone())
    }
}

pub fn nchoosek_iter(n: usize, k: usize) -> NChooseK {
    NChooseK::new(n, k)
}

pub fn gemm<T: Mul<Output = T> + Copy + Default + AddAssign>(
    a: &[Vec<T>],
    b: &[Vec<T>],
) -> Vec<Vec<T>> {
    let arows = a.len();
    let acols = a[0].len();

    let brows = b.len();
    let bcols = b[0].len();

    assert_eq!(acols, brows);

    let mut ret = vec![vec![T::default(); bcols]; arows];
    for i in 0..arows {
        for j in 0..bcols {
            (0..acols).for_each(|k| {
                ret[i][j] += a[i][k] * b[k][j];
            });
        }
    }
    ret
}

pub fn dot_vm<T: Mul<Output = T> + Copy + Default + AddAssign>(a: &[T], b: &[Vec<T>]) -> Vec<T> {
    let alen = a.len();

    let brows = b.len();
    let bcols = b[0].len();

    assert_eq!(alen, brows);

    let mut ret = vec![T::default(); bcols];
    (0..bcols).for_each(|i| {
        (0..alen).for_each(|k| {
            ret[i] += a[k] * b[k][i];
        });
    });
    ret
}

pub fn dot_mv<T: Mul<Output = T> + Copy + Default + AddAssign>(a: &[Vec<T>], b: &[T]) -> Vec<T> {
    let arows = a.len();
    let acols = a[0].len();

    let blen = b.len();

    assert_eq!(acols, blen);

    let mut ret = vec![T::default(); arows];
    (0..arows).for_each(|i| {
        (0..acols).for_each(|k| {
            ret[i] += a[i][k] * b[k];
        });
    });
    ret
}

pub fn dot<T: Mul<Output = T> + Copy + Default + Sum>(a: &[T], b: &[T]) -> T {
    a.iter().zip(b.iter()).map(|(a, b)| *a * *b).sum::<T>()
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
    fn test_lcm() {
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(1, 100), 100);
        assert_eq!(lcm(100, 1), 100);
        assert_eq!(lcm(6, 3), 6);
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(48, 56), 336);
    }

    #[test]
    fn test_number_length() {
        assert_eq!(number_length(100), 3);
        assert_eq!(number_length(999), 3);
        assert_eq!(number_length(1000), 4);
        assert_eq!(number_length(9999), 4);
    }

    #[test]
    fn test_bezout_1() {
        let (x, y) = bezout(1027, 712).unwrap();
        assert_eq!(1027 * x + 712 * y, 1);
        assert_eq!((x, y), (-165, 238)); // note: testing for a specific solution
    }

    #[test]
    fn test_bezout_2() {
        assert_eq!(bezout(4, 8), None); // not co-prime
    }

    #[test]
    fn test_bezout_3() {
        let (x, y) = bezout(8, 1).unwrap();
        assert_eq!(8 * x + y, 1);
    }

    #[test]
    fn test_bezout_4a() {
        let (x, _) = bezout(1, 0).unwrap();
        assert_eq!(x, 1);
    }

    #[test]
    fn test_bezout_4b() {
        let (_, y) = bezout(0, 1).unwrap();
        assert_eq!(y, 1);
    }

    #[test]
    fn test_bezout_4c() {
        assert_eq!(bezout(2, 0), None);
    }

    #[test]
    fn test_diophantine_1() {
        let (x, y, u, v) = diophantine(8, 1, 8).unwrap();
        for k in -10..10 {
            assert_eq!(8 * (x + k * u) + (y + k * v), 8);
        }
    }

    #[test]
    fn test_diophantine_2() {
        let (x, y, u, v) = diophantine(94, 22, 8400).unwrap();
        for k in -10..10 {
            assert_eq!(94 * (x + k * u) + 22 * (y + k * v), 8400);
        }
    }

    #[test]
    fn test_diophantine_3() {
        assert_eq!(diophantine(66, 21, 12176), None);
    }

    #[test]
    fn test_diophantine_4a() {
        let (x, y, u, v) = diophantine(4, 16, 64).unwrap();
        assert_eq!(u.abs(), 4);
        assert_eq!(v.abs(), 1);
        for k in -10..10 {
            assert_eq!(4 * (x + k * u) + 16 * (y + k * v), 64);
        }
    }

    #[test]
    fn test_diophantine_4b() {
        let (x, y, u, v) = diophantine(4, -16, 64).unwrap();
        assert_eq!(u.abs(), 4);
        assert_eq!(v.abs(), 1);
        for k in -10..10 {
            assert_eq!(4 * (x + k * u) - 16 * (y + k * v), 64);
        }
    }

    #[test]
    fn test_diophantine_4c() {
        let (x, y, u, v) = diophantine(4, 16, -64).unwrap();
        assert_eq!(u.abs(), 4);
        assert_eq!(v.abs(), 1);
        for k in -10..10 {
            assert_eq!(4 * (x + k * u) + 16 * (y + k * v), -64);
        }
    }

    #[test]
    fn test_diophantine_4d() {
        let (x, y, u, v) = diophantine(-4, -16, -64).unwrap();
        assert_eq!(u.abs(), 4);
        assert_eq!(v.abs(), 1);
        for k in -10..10 {
            assert_eq!(-4 * (x + k * u) - 16 * (y + k * v), -64);
        }
    }

    #[test]
    fn test_crt2_1() {
        let x = crt2(3, 2, 5, 3).unwrap();
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
    }

    #[test]
    fn test_crt2_2() {
        assert_eq!(crt2(3, 2, 6, 3), None);
    }

    #[test]
    fn test_crt_1() {
        let ns = vec![3, 5];
        let aa = vec![2, 3];
        let x = crt(&ns, &aa).unwrap();
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
    }

    #[test]
    fn test_crt_2() {
        let ns = vec![7, 5, 12];
        let aa = vec![3, 3, 4];
        let x = crt(&ns, &aa).unwrap();
        assert_eq!(x, 388);
    }

    #[test]
    fn test_crt_3() {
        let ns = vec![7, 5, 12, 19, 101];
        let aa = vec![3, 3, 4, 18, 81];
        let x = crt(&ns, &aa).unwrap();
        assert!(x < ns.iter().product());
        for (n, a) in ns.iter().zip(aa.iter()) {
            assert_eq!(x % n, *a);
        }
    }

    #[test]
    fn test_crt_4() {
        let ns = vec![7, 5, 12, 19, 102]; // 12 and 102 not co-prime
        let aa = vec![3, 3, 4, 18, 81];
        assert_eq!(crt(&ns, &aa), None);
    }

    #[test]
    fn test_nchoose_k() {
        let mut it = NChooseK::new(4, 2);
        assert_eq!(it.next().unwrap(), vec![0, 1]);
        assert_eq!(it.next().unwrap(), vec![0, 2]);
        assert_eq!(it.next().unwrap(), vec![0, 3]);
        assert_eq!(it.next().unwrap(), vec![1, 2]);
        assert_eq!(it.next().unwrap(), vec![1, 3]);
        assert_eq!(it.next().unwrap(), vec![2, 3]);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_nchoose_empty() {
        let mut it = NChooseK::new(4, 0);
        assert_eq!(it.next().unwrap(), Vec::<usize>::new());
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_nchoose_full() {
        let mut it = NChooseK::new(4, 4);
        assert_eq!(it.next().unwrap(), vec![0, 1, 2, 3]);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_nchoose_collect() {
        let c = nchoosek_iter(3, 2).collect::<Vec<_>>();
        assert_eq!(c, vec![vec![0, 1], vec![0, 2], vec![1, 2],]);
    }
}
