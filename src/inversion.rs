// returns inversion of permutation
pub fn inversion_perm(p: &[usize]) -> u64 {
    let incr = |i: usize, bit: &mut Vec<u64>| {
        let mut x = i + 1;
        while x < bit.len() {
            bit[x] += 1;
            x += 1 << x.trailing_zeros();
        }
    };
    let get = |i: usize, bit: &Vec<u64>| -> u64 {
        let mut x = i + 1;
        let mut ans = 0;
        while x > 0 {
            ans += bit[x];
            x -= 1 << x.trailing_zeros();
        }
        ans
    };

    let n = p.len();
    let mut bit = vec![0; n + 1];
    let mut ans = 0;
    for i in (0..n).rev() {
        ans += get(p[i], &bit);
        incr(p[i], &mut bit);
    }

    ans
}

// returns #{(i, j) | a[i] > a[j]}
pub fn inversion<T: Ord>(a: &[T]) -> u64 {
    use itertools::Itertools;
    let n = a.len();
    // sorted_by_key is stable
    let q = (0..n).sorted_by_key(|&i| &a[i]).collect_vec();
    let mut p = vec![0; n];
    for (i, qi) in q.into_iter().enumerate() {
        p[qi] = i;
    }
    inversion_perm(&p)
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::*;

    fn bruteforce(p: &[usize]) -> u64 {
        (0..p.len())
            .tuple_combinations()
            .map(|(i, j)| if p[i] > p[j] { 1 } else { 0 })
            .sum::<u64>()
    }

    #[test]
    fn test_small_perm() {
        assert_eq!(inversion_perm(&vec![0]), 0);
        assert_eq!(inversion_perm(&vec![0, 1]), 0);
        assert_eq!(inversion_perm(&vec![1, 0]), 1);
        assert_eq!(inversion_perm(&vec![0, 1, 2]), 0);
        assert_eq!(inversion_perm(&vec![0, 2, 1]), 1);
        assert_eq!(inversion_perm(&vec![1, 0, 2]), 1);
        assert_eq!(inversion_perm(&vec![1, 2, 0]), 2);
        assert_eq!(inversion_perm(&vec![2, 0, 1]), 2);
        assert_eq!(inversion_perm(&vec![2, 1, 0]), 3);
        for p in (0..4).permutations(4) {
            assert_eq!(inversion_perm(&p), bruteforce(&p));
        }
        for p in (0..5).permutations(5) {
            assert_eq!(inversion_perm(&p), bruteforce(&p));
        }
    }

    #[test]
    fn test_small() {
        let a = vec![6, 8, 2, 9, 3];
        for n in 1..=a.len() {
            for p in (0..n).permutations(n) {
                let b = p.iter().map(|&i| a[i]).collect::<Vec<_>>();
                assert_eq!(inversion(&b), bruteforce(&b));
            }
        }
    }

    #[test]
    fn test_small_dup() {
        let a = vec![5, 4, 4, 6, 8];
        for n in 1..=a.len() {
            for p in (0..n).permutations(n) {
                let b = p.iter().map(|&i| a[i]).collect::<Vec<_>>();
                assert_eq!(inversion(&b), bruteforce(&b));
            }
        }
    }
}
