use num_traits::{One, Zero};
use std::ops::{Add, Mul};

pub fn matrix_multiply<T: Copy + PartialEq + Zero + One + Add + Mul>(
    a: &Vec<Vec<T>>,
    b: &Vec<Vec<T>>,
) -> Vec<Vec<T>> {
    let mut ans = vec![vec![T::zero(); a.len()]; b[0].len()];
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..b.len() {
                ans[i][j] = ans[i][j] + a[i][k] * b[k][j];
            }
        }
    }
    ans
}

pub fn matrix_pow<T: Copy + PartialEq + Zero + One + Add + Mul>(
    m: &Vec<Vec<T>>,
    n: i64,
) -> Vec<Vec<T>> {
    let mut temp = vec![vec![T::zero(); m.len()]; m.len()];
    for i in 0..m.len() {
        temp[i][i] = T::one();
    }
    let mut power = (*m).clone();
    let mut e = n;
    while e > 0 {
        if e % 2 == 1 {
            temp = matrix_multiply(&temp, &power);
        }
        power = matrix_multiply(&power, &power);
        e /= 2;
    }
    temp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply_2x2_i32() {
        let a: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let b: Vec<Vec<i32>> = vec![vec![5, 6], vec![7, 8]];

        let result = matrix_multiply(&a, &b);

        assert_eq!(result[0][0], 19); // 1*5 + 2*7
        assert_eq!(result[0][1], 22); // 1*6 + 2*8
        assert_eq!(result[1][0], 43); // 3*5 + 4*7
        assert_eq!(result[1][1], 50); // 3*6 + 4*8
    }

    #[test]
    fn test_matrix_multiply_3x3_i64() {
        let a: Vec<Vec<i64>> =
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let b: Vec<Vec<i64>> =
            vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];

        let result = matrix_multiply(&a, &b);

        assert_eq!(result[0][0], 30); // 1*9 + 2*6 + 3*3
        assert_eq!(result[0][1], 24); // 1*8 + 2*5 + 3*2
        assert_eq!(result[0][2], 18); // 1*7 + 2*4 + 3*1
        assert_eq!(result[1][0], 84); // 4*9 + 5*6 + 6*3
        assert_eq!(result[1][1], 69); // 4*8 + 5*5 + 6*2
        assert_eq!(result[1][2], 54); // 4*7 + 5*4 + 6*1
        assert_eq!(result[2][0], 138); // 7*9 + 8*6 + 9*3
        assert_eq!(result[2][1], 114); // 7*8 + 8*5 + 9*2
        assert_eq!(result[2][2], 90); // 7*7 + 8*4 + 9*1
    }

    #[test]
    fn test_matrix_pow_power_of_zero() {
        let m: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let result = matrix_pow(&m, 0);

        // Result should be identity matrix
        assert_eq!(result[0][0], 1);
        assert_eq!(result[0][1], 0);
        assert_eq!(result[1][0], 0);
        assert_eq!(result[1][1], 1);
    }

    #[test]
    fn test_matrix_pow_power_of_one() {
        let m: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let result = matrix_pow(&m, 1);

        // Result should be the same matrix
        assert_eq!(result[0][0], 1);
        assert_eq!(result[0][1], 2);
        assert_eq!(result[1][0], 3);
        assert_eq!(result[1][1], 4);
    }

    #[test]
    fn test_matrix_pow_power_of_two_i32() {
        let m: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let result = matrix_pow(&m, 2);

        // [[1, 2], [3, 4]]^2 = [[7, 10], [15, 22]]
        assert_eq!(result[0][0], 7);
        assert_eq!(result[0][1], 10);
        assert_eq!(result[1][0], 15);
        assert_eq!(result[1][1], 22);
    }

    #[test]
    fn test_matrix_pow_power_of_three_i64() {
        let m: Vec<Vec<i64>> = vec![vec![1, 1], vec![1, 0]];
        let result = matrix_pow(&m, 3);

        // Fibonacci matrix [[1, 1], [1, 0]]^3 = [[3, 2], [2, 1]]
        assert_eq!(result[0][0], 3);
        assert_eq!(result[0][1], 2);
        assert_eq!(result[1][0], 2);
        assert_eq!(result[1][1], 1);
    }

    #[test]
    fn test_matrix_pow_single_element() {
        let m: Vec<Vec<i32>> = vec![vec![5]];
        let result = matrix_pow(&m, 3);

        // [[5]]^3 = [[125]]
        assert_eq!(result[0][0], 125);
    }

    #[test]
    fn test_matrix_pow_identity_matrix() {
        let m: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1]];
        let result = matrix_pow(&m, 100);

        // Identity^n = Identity
        assert_eq!(result[0][0], 1);
        assert_eq!(result[0][1], 0);
        assert_eq!(result[1][0], 0);
        assert_eq!(result[1][1], 1);
    }

    #[test]
    fn test_matrix_multiply_identity() {
        let a: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let identity: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1]];

        let result = matrix_multiply(&a, &identity);

        // A * I = A
        assert_eq!(result[0][0], 1);
        assert_eq!(result[0][1], 2);
        assert_eq!(result[1][0], 3);
        assert_eq!(result[1][1], 4);
    }
}
