use ac_library::ModInt998244353 as Mint;

/// Lagrange interpolation
/// Given a set of (x, y) values, compute the coefficients of the interpolating polynomial
pub fn interpolate(values: &Vec<(i64, Mint)>) -> Vec<Mint> {
    let n = values.len();
    let mut result = vec![Mint::new(0); n];
    for (x, y) in values {
        let mut coeff = *y;
        let mut term = vec![Mint::new(0); n];
        term[0] = Mint::new(1);
        for (x1, _) in values {
            if x != x1 {
                coeff /= x - x1;
                for i in (0..n).rev() {
                    term[i] *= -x1;
                    if i > 0 {
                        let tmp = term[i - 1];
                        term[i] += tmp;
                    }
                }
            }
        }

        for i in 0..n {
            result[i] += term[i] * coeff;
        }
    }

    result
}

/// Polynomial evaluation using Horner's method
pub fn evaluate(p: &Vec<Mint>, x: i64) -> Mint {
    let mut result = Mint::new(0);
    for c in p.iter().rev() {
        result = result * x + c;
    }
    result
}
