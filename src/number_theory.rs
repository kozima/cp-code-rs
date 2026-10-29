/// Modular exponentiation: computes (b^e) % m
pub fn mod_exp(b: u64, e: u64, m: u64) -> u64 {
    let mut result = 1;
    let mut power = b;
    let mut e = e;
    while e > 0 {
        if e % 2 == 1 {
            result = result * power % m;
        }
        e /= 2;
        power = power * power % m;
    }

    result
}

/// Moebius function mu(n) for all n < size
/// Returns a vector where mu[i] is the Moebius function value of i
pub fn moebius_function(n: usize) -> Vec<i64> {
    let mut mu = vec![1; n];
    let mut is_prime = vec![true; n];
    for x in 2..n as usize {
        if is_prime[x] {
            mu[x] *= -1;
            for i in 2.. {
                if i * x >= n {
                    break;
                }
                is_prime[i * x] = false;
                mu[i * x] *= -1;
            }
            if x <= (n - 1) / x {
                // x * x < n
                for i in 1.. {
                    if x * x <= (n - 1) / i {
                        // i * x * x < n
                        mu[i * x * x] = 0;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    mu
}

/// Floor of nth root: returns the largest integer r such that r^n <= x
pub fn floor_nth_root(x: u64, n: u32) -> u64 {
    assert!(n != 0);
    if x < 2 {
        return x;
    }
    if n == 1 {
        return x;
    }
    let mut ans = (x as f64).powf(1.0 / n as f64).ceil() as u64;
    while (ans as i128).pow(n) > x as i128 {
        ans -= 1;
    }
    ans
}

#[cfg(test)]
mod test_floor_nth_root {
    use super::floor_nth_root;

    #[test]
    fn test_floor_nth_root_small() {
        for n in 1..60 {
            for x in 0..10000 {
                let r = floor_nth_root(x, n);
                assert!(r.pow(n) <= x);
                assert!((r + 1).pow(n) > x);
            }
        }
    }

    #[test]
    fn test_floor_nth_root_large() {
        use rand::*;
        use rand_pcg::Pcg32;
        let mut rng = Pcg32::seed_from_u64(0);
        for _ in 0..100_000 {
            let x = rng.random_range(0..u64::MAX);
            for n in 1..63 {
                let r = floor_nth_root(x, n);
                assert!(r.pow(n) <= x);
                assert!((r as u128 + 1).pow(n) > x as u128);
            }
        }
    }
}
