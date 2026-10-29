/// Binomial coefficient C(n, k)
pub fn choose(n: u32, k: u32) -> i64 {
    assert!(k <= n);
    let mut a = 1;
    for i in 2..=n {
        a *= i as i64;
    }
    for i in 2..=k {
        a /= i as i64;
    }
    for i in 2..=n - k {
        a /= i as i64;
    }
    a
}

/// Harmonic number H(n) = 1 + 1/2 + 1/3 + ... + 1/n
pub fn harmonic_number(n: i64) -> f64 {
    // (n as f64).ln() + GAMMA is better for n >= 10^13
    // e = 10
    // 23.603066594892102  <- better
    // 23.60306659484199
    // e = 11
    // 25.90565168784115   <- better
    // 25.905651687836038
    // e = 12
    // 28.208236780830696  <- better
    // 28.20823678083008
    // e = 13
    // 30.51082187382429
    // 30.510821873824128  <- better
    // e = 14
    // 32.81340696681829
    // 32.81340696681817   <- better
    // e = 15
    // 35.11599205981233
    // 35.115992059812214  <- better
    const N0: i64 = 10_i64.pow(7);
    const N1: i64 = 10_i64.pow(13);
    const GAMMA: f64 = 0.57721566490153286;
    if n < N0 {
        (1..=n).map(|i| 1.0 / i as f64).sum::<f64>()
    } else if n < N1 {
        let pref_sum = (1..N0).rev().map(|i| 1.0 / i as f64).sum::<f64>();
        pref_sum + ((n as f64 + 0.5) / (N0 as f64 - 0.5)).ln()
    } else {
        (n as f64).ln() + GAMMA
    }
}
