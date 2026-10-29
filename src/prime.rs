/// Linear sieve: returns all primes up to n
pub fn list_primes(n: i64) -> Vec<i64> {
    let mut primes = vec![];
    let mut lpf = vec![1; n as usize + 1];
    for x in 2..=n as usize {
        if lpf[x] == 1 {
            lpf[x] = x as i64;
            primes.push(x as i64);
        }
        for &p in &primes {
            if p * (x as i64) <= n && p <= lpf[x] {
                lpf[p as usize * x] = p;
            } else {
                break;
            }
        }
    }
    primes
}

/// Prime factorization of m
/// Returns a vector of (prime, exponent) pairs
pub fn prime_factorize(m: u64) -> Vec<(u64, u32)> {
    let mut factors = vec![];
    let mut tmp = m;
    for p in 2.. {
        if p * p > tmp {
            break;
        }
        let mut e = 0;
        while tmp % p == 0 {
            e += 1;
            tmp /= p;
        }
        if e > 0 {
            factors.push((p, e));
        }
    }
    if tmp > 1 {
        factors.push((tmp, 1));
    }
    factors
}
