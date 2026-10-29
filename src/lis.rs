pub fn lis_len<T: Ord>(xs: &[T]) -> usize {
    let mut dp = vec![];
    for x in xs {
        let i = dp.partition_point(|&y| y < x);
        if i == dp.len() {
            dp.push(x);
        } else {
            dp[i] = x;
        }
    }
    dp.len()
}

// returns indices
pub fn lis<T: Ord>(xs: &[T]) -> Vec<usize> {
    let mut dp = vec![];
    let mut prev = vec![None; xs.len()];
    for (i, x) in xs.iter().enumerate() {
        let j = dp.partition_point(|&y| &xs[y] < x);
        if j == dp.len() {
            dp.push(i);
            prev[i] = if j == 0 { None } else { Some(dp[j - 1]) };
        } else {
            dp[j] = i;
            prev[i] = if j == 0 { None } else { Some(dp[j - 1]) };
        }
    }
    let mut res = vec![];
    res.reserve(dp.len());
    let mut i = dp.last().copied();
    while let Some(j) = i {
        res.push(j);
        i = prev[j];
    }
    res.reverse();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(lis_len::<usize>(&vec![]), 0);
        assert_eq!(lis_len(&vec![3]), 1);
        assert_eq!(lis_len(&vec![3, 4]), 2);
        assert_eq!(lis_len(&vec![4, 3]), 1);
        assert_eq!(lis_len(&vec![3, 4, 6]), 3);
        assert_eq!(lis_len(&vec![3, 6, 4]), 2);
        assert_eq!(lis_len(&vec![4, 3, 6]), 2);
        assert_eq!(lis_len(&vec![4, 6, 3]), 2);
        assert_eq!(lis_len(&vec![6, 3, 4]), 2);
        assert_eq!(lis_len(&vec![6, 4, 3]), 1);
        assert_eq!(lis_len(&vec![3, 3]), 1);
        assert_eq!(lis_len(&vec![3, 3, 3]), 1);

        assert_eq!(lis::<usize>(&vec![]), vec![]);
        assert_eq!(lis(&vec![3]), vec![0]);
        assert_eq!(lis(&vec![3, 4]), vec![0, 1]);

        let res = lis(&vec![4, 3]);
        assert!(res == vec![0] || res == vec![1]);

        assert_eq!(lis(&vec![3, 4, 6]), vec![0, 1, 2]);

        let res = lis(&vec![3, 6, 4]);
        assert!(res == vec![0, 1] || res == vec![0, 2]);

        let res = lis(&vec![4, 3, 6]);
        assert!(res == vec![0, 2] || res == vec![1, 2]);

        assert_eq!(lis(&vec![4, 6, 3]), vec![0, 1]);

        assert_eq!(lis(&vec![6, 3, 4]), vec![1, 2]);

        let res = lis(&vec![6, 4, 3]);
        assert!(res == vec![0] || res == vec![1] || res == vec![2]);
    }
}
