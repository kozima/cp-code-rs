pub fn largest_differencing_method_2(a: &Vec<i64>) -> Vec<usize> {
    #[derive(Debug, PartialEq, Eq)]
    struct Config {
        lhs: Vec<i64>,
        rhs: Vec<i64>,
        diff: u64,
    }

    impl Ord for Config {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.diff.cmp(&other.diff)
        }
    }

    impl PartialOrd for Config {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(&other))
        }
    }

    let n = a.len();
    let mut configs = a
        .iter()
        .map(|&x| {
            if x >= 0 {
                Config {
                    lhs: vec![x],
                    rhs: vec![],
                    diff: x.abs() as u64,
                }
            } else {
                Config {
                    lhs: vec![],
                    rhs: vec![x],
                    diff: x.abs() as u64,
                }
            }
        })
        .collect::<std::collections::BinaryHeap<Config>>();

    while configs.len() > 1 {
        let c1 = configs.pop().unwrap();
        let c2 = configs.pop().unwrap();
        let (c1, c2) = if c1.diff >= c2.diff {
            (c1, c2)
        } else {
            (c2, c1)
        };

        let diff = c1.diff - c2.diff;
        let Config {
            mut lhs, mut rhs, ..
        } = c1;
        let Config {
            lhs: mut lhs2,
            rhs: mut rhs2,
            ..
        } = c2;
        lhs.append(&mut rhs2);
        rhs.append(&mut lhs2);
        let c = Config { lhs, rhs, diff };
        configs.push(c);
    }

    // let error = configs[0].groups.iter()
    //     .map(|p| p.sum.abs())
    //     .sum::<i64>();

    let mut used = vec![false; n];
    let mut ret = vec![0; n];
    let Config { lhs, rhs, .. } = configs.pop().unwrap();
    for (values, index) in [(lhs, 0), (rhs, 1)] {
        for x in values {
            for i in 0..n {
                if !used[i] && a[i] == x {
                    used[i] = true;
                    ret[i] = index;
                    break;
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod test {
    // use rand::*;
    // use rand_pcg::Pcg32;
    // use itertools::Itertools;

    // struct Timer {
    //     start: std::time::SystemTime
    // }

    // impl Timer {
    //     fn new() -> Timer {
    //         let start = std::time::SystemTime::now();
    //         Timer {start}
    //     }

    //     fn get_elapsed_time(&self) -> u64 {
    //         let dur = std::time::SystemTime::now().duration_since(self.start).unwrap();
    //         dur.as_millis() as u64
    //     }
    // }

    #[test]
    fn test_ldm_2() {
        let v = vec![1, 3, 5, 10];
        let ans = crate::partition::largest_differencing_method_2(&v);
        assert!(ans == vec![0, 0, 0, 1] || ans == vec![1, 1, 1, 0]);
    }
}

pub fn largest_differencing_method(a: &Vec<i64>, k: usize) -> Vec<usize> {
    #[derive(Debug, Clone)]
    struct Group {
        sum: i64,
        values: Vec<i64>,
    }
    #[derive(Debug)]
    struct Config {
        diff: u64,
        groups: Vec<Group>,
    }

    let n = a.len();
    let mut configs = vec![];
    for i in 0..n {
        let empty_group = Group {
            sum: 0,
            values: vec![],
        };
        let mut groups = vec![empty_group; k - 1];
        let group = Group {
            sum: a[i],
            values: vec![a[i]],
        };
        if a[i] > 0 {
            groups.push(group);
        } else {
            groups.insert(0, group);
        }
        assert_eq!(groups.len(), k);
        let diff = a[i].abs() as u64;
        configs.push(Config { diff, groups });
    }

    while configs.len() > 1 {
        configs.sort_by_key(|c| c.diff);
        let mut c1 = configs.pop().unwrap();
        let c2 = configs.pop().unwrap();
        c1.groups.reverse();
        let mut groups = vec![];
        for (p1, p2) in c1.groups.iter().zip(c2.groups.iter()) {
            let sum = p1.sum + p2.sum;
            let mut values = p1.values.clone();
            values.append(&mut p2.values.clone());
            groups.push(Group { sum, values });
        }
        groups.sort_by_key(|p| p.sum);
        assert_eq!(groups.len(), k);

        let diff = groups
            .iter()
            .map(|p| p.sum)
            .max()
            .unwrap()
            .abs_diff(groups.iter().map(|p| p.sum).min().unwrap());
        configs.push(Config { diff, groups });
    }

    // let error = configs[0].groups.iter()
    //     .map(|p| p.sum.abs())
    //     .sum::<i64>();

    let mut used = vec![false; n];
    let mut ret = vec![0; n];
    for (j, group) in configs[0].groups.iter().enumerate() {
        for &c in &group.values {
            for i in 0..n {
                if !used[i] && a[i] == c {
                    used[i] = true;
                    ret[i] = j;
                    break;
                }
            }
        }
    }

    ret
}

#[derive(Clone, Copy)]
pub enum Method {
    FFD,
    BFD,
}

fn try_fill(
    c: &Vec<i64>,
    k: usize,
    cap: i64,
    method: Method,
) -> Option<Vec<usize>> {
    match method {
        Method::FFD => try_fill_ffd(c, k, cap),
        Method::BFD => try_fill_bfd(c, k, cap),
    }
}

pub fn try_fill_bfd(c: &Vec<i64>, k: usize, cap: i64) -> Option<Vec<usize>> {
    let mut rem_cap = vec![cap; k];
    let mut ans = vec![0; c.len()];
    for i in 0..c.len() {
        if let Some(j) = (0..k)
            .filter(|&j| rem_cap[j] >= c[i])
            .min_by_key(|&j| rem_cap[j])
        {
            ans[i] = j;
            rem_cap[j] -= c[i];
        } else {
            return None;
        }
    }
    Some(ans)
}

pub fn try_fill_ffd(c: &Vec<i64>, k: usize, cap: i64) -> Option<Vec<usize>> {
    let mut rem_cap = vec![cap; k];
    let mut ans = vec![0; c.len()];
    for i in 0..c.len() {
        if let Some(j) = (0..k).find(|&j| rem_cap[j] >= c[i]) {
            ans[i] = j;
            rem_cap[j] -= c[i];
        } else {
            return None;
        }
    }
    Some(ans)
}

pub fn multifit(c: &Vec<i64>, k: usize, method: Method) -> Vec<usize> {
    use itertools::Itertools;
    let c_with_index = c
        .clone()
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, v)| std::cmp::Reverse(v))
        .collect_vec();
    let orig_index = c_with_index.iter().map(|(i, _)| i).copied().collect_vec();
    let c_sorted = c_with_index.iter().map(|(_, v)| v).copied().collect_vec();

    let mut lb = (c.iter().sum::<i64>() - 1) / k as i64 + 1;
    let mut ub = lb + 1;
    let mut ans = loop {
        if let Some(a) = try_fill(&c_sorted, k, ub, method) {
            break a;
        } else {
            ub += ub - lb;
        }
    };
    while ub - lb > 1 {
        let m = (ub + lb) / 2;
        if let Some(a) = try_fill(&c_sorted, k, m, method) {
            ans = a;
            ub = m;
        } else {
            lb = m;
        }
    }

    let mut ret = vec![0; c.len()];
    for (i, j) in ans.iter().enumerate() {
        ret[orig_index[i]] = *j;
    }
    ret
}
