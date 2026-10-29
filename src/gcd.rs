// assume a, b != i64::MIN
// (a, b) -> (g, x, y) such that
//   - a * x + b * y = g = gcd(a, b)
//   - |x| < |b|
//   - |y| < |a|
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    assert!(a != i64::MIN);
    assert!(b != i64::MIN);
    let (mut x, mut g) = if a > 0 { (1, a) } else { (-1, -a) };
    let (mut w, mut h) = if b > 0 { (1, b) } else { (-1, -b) };
    let mut y = 0;
    let mut z = 0;
    // [[x y] [z w]] * [a b] = [g h]
    while h != 0 {
        let q = g / h;
        let tmp = x - q * z;
        x = z;
        z = tmp;
        let tmp = y - q * w;
        y = w;
        w = tmp;
        let tmp = g - q * h;
        g = h;
        h = tmp;
        assert_eq!(a as i128 * x as i128 + b as i128 * y as i128, g as i128);
        assert_eq!(a as i128 * z as i128 + b as i128 * w as i128, h as i128);
    }

    assert!(g >= 0);
    (g, x, y)
}

#[cfg(test)]
mod test {
    use crate::gcd::extended_gcd;
    use num_integer::gcd;
    use rand::*;
    use rand_pcg::Pcg32;

    #[test]
    fn test_zero() {
        for x in 0..100 {
            let (g, a, b) = extended_gcd(x, 0);
            assert_eq!(g, x);
            assert_eq!(g, a * x);
            assert_eq!(b, 0);
        }
        for x in 1..100 {
            let (g, a, b) = extended_gcd(-x, 0);
            assert_eq!(g, x);
            assert_eq!(g, -a * x);
            assert_eq!(b, 0);
        }
    }

    #[test]
    fn test_small() {
        for x in (-10..0).chain(1..=10) {
            for y in (-10..0).chain(1..=10) {
                let (g, a, b) = extended_gcd(x, y);
                assert_eq!(g, gcd(x, y));
                assert_eq!(g, a * x + b * y);
                assert!(a.abs() <= y.abs());
                assert!(b.abs() <= x.abs());
            }
        }
    }

    const MIN_VALUE: i64 = i64::MIN + 1;
    const MAX_VALUE: i64 = i64::MAX;

    #[test]
    fn test_rand() {
        let mut rng = Pcg32::seed_from_u64(0);
        for _ in 0..100000 {
            let x = rng.random_range(MIN_VALUE..=MAX_VALUE);
            let y = rng.random_range(MIN_VALUE..=MAX_VALUE);
            let (g, a, b) = extended_gcd(x, y);
            assert_eq!(g, gcd(x, y));
            assert_eq!(
                g as i128,
                a as i128 * x as i128 + b as i128 * y as i128
            );
            assert!(a.abs() <= y.abs());
            assert!(b.abs() <= x.abs());
        }
    }

    #[test]
    fn test_min() {
        let mut rng = Pcg32::seed_from_u64(0);
        for _ in 0..100 {
            let x = MIN_VALUE;
            let y = rng.random_range(MIN_VALUE..=MAX_VALUE);
            let (g, a, b) = extended_gcd(x, y);
            assert_eq!(g, gcd(x, y));
            assert_eq!(
                g as i128,
                a as i128 * x as i128 + b as i128 * y as i128
            );
            assert!(a.abs() <= y.abs());
            assert!(b.abs() <= x.abs());
        }
        for _ in 0..100 {
            let x = rng.random_range(MIN_VALUE..=MAX_VALUE);
            let y = MIN_VALUE;
            let (g, a, b) = extended_gcd(x, y);
            assert_eq!(g, gcd(x, y));
            assert_eq!(
                g as i128,
                a as i128 * x as i128 + b as i128 * y as i128
            );
            assert!(a.abs() <= y.abs());
            assert!(b.abs() <= x.abs());
        }
    }

    #[test]
    fn test_max() {
        let mut rng = Pcg32::seed_from_u64(0);
        for _ in 0..100 {
            let x = MAX_VALUE;
            let y = rng.random_range(MIN_VALUE..=MAX_VALUE);
            let (g, a, b) = extended_gcd(x, y);
            assert_eq!(g, gcd(x, y));
            assert_eq!(
                g as i128,
                a as i128 * x as i128 + b as i128 * y as i128
            );
            assert!(a.abs() <= y.abs());
            assert!(b.abs() <= x.abs());
        }
        for _ in 0..100 {
            let x = rng.random_range(MIN_VALUE..=MAX_VALUE);
            let y = MAX_VALUE;
            let (g, a, b) = extended_gcd(x, y);
            assert_eq!(g, gcd(x, y));
            assert_eq!(
                g as i128,
                a as i128 * x as i128 + b as i128 * y as i128
            );
            assert!(a.abs() <= y.abs());
            assert!(b.abs() <= x.abs());
        }
    }
}
