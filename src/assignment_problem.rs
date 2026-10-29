use itertools::*;

pub trait CostType:
    Clone
    + Copy
    + Ord
    + num::traits::Zero
    + std::ops::Sub<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::fmt::Debug
    + std::fmt::Display
{
    const MAX: Self;
}

macro_rules! impl_cost_type {
    ($($t:ty),* $(,)?) => {
        $(
            impl CostType for $t {
                const MAX: Self = Self::MAX;
            }
        )*
    };
}

impl_cost_type!(i8, i16, i32, i64, i128, isize);

pub struct Instance<Cost: CostType> {
    n: usize,
    cost: Vec<Vec<Cost>>,
}

impl<Cost: CostType> Instance<Cost> {
    pub fn new(n: usize, cost: Vec<Vec<Cost>>) -> Self {
        Self { n, cost }
    }
}

pub struct Solution<Cost: CostType> {
    pub matching: Vec<(usize, usize)>,
    pub cost: Cost,
}

// Hungarian algorithm

struct HungarianAlgorithmImpl<'a, Cost: CostType> {
    instance: &'a Instance<Cost>,

    // matching between 0..i and 0..n
    match_lr: Vec<Option<usize>>,
    match_rl: Vec<Option<usize>>,

    potential_l: Vec<Cost>,
    potential_r: Vec<Cost>,
}

impl<'a, Cost: CostType> HungarianAlgorithmImpl<'a, Cost> {
    #[allow(unused)]
    fn check_invariant(self: &Self) {
        let n = self.instance.n;
        assert_eq!(self.match_lr.len(), n);
        assert_eq!(self.match_rl.len(), n);
        assert_eq!(self.potential_l.len(), n);
        assert_eq!(self.potential_r.len(), n);

        // check matching
        let u_max = (0..n).filter(|&u| self.match_lr[u].is_some()).count();
        for u in 0..u_max {
            assert!(self.match_lr[u].is_some());
            let v = self.match_lr[u].unwrap();
            assert_eq!(self.match_rl[v], Some(u));
        }
        for u in u_max..n {
            assert!(self.match_lr[u].is_none());
        }

        // edges in matching are tight
        for u in 0..n {
            if let Some(v) = self.match_lr[u] {
                assert!(self.is_tight(u, v));
            }
        }

        // check potential
        for (u, v) in iproduct!(0..n, 0..n) {
            assert!(self.slack(u, v) >= Cost::zero());
        }
    }

    fn new(instance: &'a Instance<Cost>) -> Self {
        let n = instance.n;
        let potential_l = instance
            .cost
            .iter()
            .map(|ci| ci.iter().min().unwrap())
            .copied()
            .collect_vec();

        Self {
            instance,
            match_lr: vec![None; n],
            match_rl: vec![None; n],
            potential_l,
            potential_r: vec![Cost::zero(); n],
        }
    }

    fn is_tight(self: &Self, u: usize, v: usize) -> bool {
        self.instance.cost[u][v] == self.potential_l[u] + self.potential_r[v]
    }

    fn slack(self: &Self, u: usize, v: usize) -> Cost {
        self.instance.cost[u][v] - self.potential_l[u] - self.potential_r[v]
    }

    fn augment_matching(
        self: &mut Self,
        state: SearchState<Cost>,
        from: usize,
        to: usize,
    ) {
        let mut v = to;
        loop {
            let u = state.get_parent_r(v).unwrap();
            let next_v = self.match_lr[u];
            self.match_lr[u] = Some(v);
            self.match_rl[v] = Some(u);
            if u == from {
                assert!(next_v.is_none());
                break;
            } else {
                assert!(next_v.is_some());
                v = next_v.unwrap();
            }
        }
    }

    fn search_unmatched_node(
        self: &mut Self,
        state: &mut SearchState<Cost>,
    ) -> Option<usize> {
        while let Some(u) = state.stack.pop() {
            for v in 0..self.instance.n {
                if !state.is_visited_r(v) {
                    if self.is_tight(u, v) {
                        state.visit_r_from(u, v);
                        if let Some(u1) = self.match_rl[v] {
                            state.visit_l(u1);
                            state.stack.push(u1);
                        } else {
                            return Some(v);
                        }
                    } else {
                        state.min_slack_r[v] =
                            state.min_slack_r[v].min((self.slack(u, v), u));
                    }
                }
            }
        }

        None
    }

    // update_potential の直後に呼び出される
    fn get_unmatched_node_if_exists(
        self: &Self,
        state: &mut SearchState<Cost>,
    ) -> Option<usize> {
        for v in 0..self.instance.n {
            if !state.is_visited_r(v) {
                if state.min_slack_r[v].0 == Cost::zero() {
                    let p = state.min_slack_r[v].1;
                    state.visit_r_from(p, v);
                    if let Some(u) = self.match_rl[v] {
                        state.visit_l(u);
                    } else {
                        return Some(v);
                    }
                }
            }
        }

        None
    }

    fn update_potential(self: &mut Self, state: &mut SearchState<Cost>) {
        let n = self.instance.n;

        let (delta, p) = (0..n)
            .filter(|&v| !state.is_visited_r(v))
            .map(|v| state.min_slack_r[v])
            .min()
            .unwrap();

        assert!(state.is_visited_l(p));
        assert!(delta > Cost::zero());
        for u in 0..n {
            if state.is_visited_l(u) {
                self.potential_l[u] += delta;
            }
        }
        for v in 0..n {
            if state.is_visited_r(v) {
                self.potential_r[v] -= delta;
            } else {
                state.min_slack_r[v].0 -= delta;
            }
        }
    }

    // root: matching に追加したい左頂点
    // - root から tight edge をたどった tree を作る
    // - unmatched な右頂点が見つかったらそれ使って matching を拡張する
    // - unmatched な右頂点に到達できない場合、到達可能左頂点と到達不
    //   可能右頂点の間の辺のうち slack 最小の辺をとり、それが tight に
    //   なるよう potential を更新して続きを探索する
    fn extend(self: &mut Self, root: usize) {
        assert!(self.match_lr[root].is_none());
        let mut state = SearchState::new(root, self.instance.n);
        loop {
            let vopt = self.search_unmatched_node(&mut state);

            if let Some(v) = vopt {
                self.augment_matching(state, root, v);
                break;
            } else {
                // update potential, and augment matching if possible
                self.update_potential(&mut state);
                if let Some(v) = self.get_unmatched_node_if_exists(&mut state) {
                    self.augment_matching(state, root, v);
                    break;
                }
            }
        }

        assert!(self.match_lr[root].is_some());
    }

    fn solve(mut self: Self) -> Solution<Cost> {
        let n = self.instance.n;
        for u in 0..n {
            self.extend(u);

            #[cfg(debug_assertions)]
            self.check_invariant();
        }

        let mut matching = vec![];
        matching.reserve(n);
        let mut cost = Cost::zero();
        for u in 0..n {
            let v = self.match_lr[u].unwrap();
            matching.push((u, v));
            cost += self.instance.cost[u][v];
        }

        Solution { matching, cost }
    }
}

struct SearchState<Cost: CostType> {
    stack: Vec<usize>,
    visited_l: Vec<bool>,
    parent_r: Vec<Option<usize>>, // v -> v の直前の左頂点
    min_slack_r: Vec<(Cost, usize)>,
}

impl<Cost: CostType> SearchState<Cost> {
    fn is_visited_l(self: &Self, u: usize) -> bool {
        self.visited_l[u]
    }

    fn is_visited_r(self: &Self, v: usize) -> bool {
        self.parent_r[v].is_some()
    }

    fn get_parent_r(self: &Self, v: usize) -> Option<usize> {
        self.parent_r[v]
    }

    fn visit_r_from(self: &mut Self, u: usize, v: usize) {
        self.parent_r[v] = Some(u);
    }

    fn visit_l(self: &mut Self, u: usize) {
        assert_eq!(self.visited_l[u], false);
        self.visited_l[u] = true;
        self.stack.push(u);
    }

    fn new(root: usize, n: usize) -> Self {
        let mut visited_l = vec![false; n];
        visited_l[root] = true;
        SearchState {
            stack: vec![root],
            visited_l,
            parent_r: vec![None; n],
            min_slack_r: vec![(Cost::MAX, 0); n],
        }
    }
}

pub fn hungarian_algorithm<Cost: CostType>(
    instance: &Instance<Cost>,
) -> Solution<Cost> {
    HungarianAlgorithmImpl::new(instance).solve()
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use rand::*;
    use rand_pcg::Pcg32;

    struct Timer {
        start: std::time::SystemTime,
    }

    impl Timer {
        fn new() -> Timer {
            let start = std::time::SystemTime::now();
            Timer { start }
        }

        fn get_elapsed_time(&self) -> u64 {
            let dur = std::time::SystemTime::now()
                .duration_since(self.start)
                .unwrap();
            dur.as_millis() as u64
        }
    }

    use super::*;

    #[test]
    fn test1() {
        let instance = Instance {
            n: 1,
            cost: vec![vec![2]],
        };

        let solution = hungarian_algorithm(&instance);
        // eprintln!("{:?}", solution.matching);
        // eprintln!("cost = {}", solution.cost);
        assert_eq!(solution.cost, 2);
    }

    #[test]
    fn test2() {
        let instance = Instance {
            n: 2,
            cost: vec![vec![3, 2], vec![1, 5]],
        };

        let solution = hungarian_algorithm(&instance);
        // eprintln!("{:?}", solution.matching);
        // eprintln!("cost = {}", solution.cost);
        assert_eq!(solution.cost, 3);
    }

    #[test]
    fn test_rand() {
        // let cost = vec![vec![3, 8, 1], vec![2, 9, 9], vec![5, 3, 1]];
        // eprintln!("{:?}", cost);
        // let instance = Instance { n: 3, cost };
        // let solution = hungarian_algorithm(&instance);

        let mut rng = Pcg32::seed_from_u64(0);
        for n in 3..=12 {
            for _ in 0..100 {
                let cost = (0..n)
                    .map(|_| {
                        (0..n).map(|_| rng.random_range(1..10)).collect_vec()
                    })
                    .collect_vec();
                // eprintln!("{:?}", cost);
                let instance = Instance { n, cost };

                let solution = hungarian_algorithm(&instance);
                // eprintln!("{:?}", solution.matching);
                // eprintln!("cost = {}", solution.cost);
                let expected = bruteforce(&instance);
                assert_eq!(solution.cost, expected);
            }
        }
    }

    #[test]
    fn test_large() {
        for n in [100, 200, 400, 800] {
            let cost = (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| (i + 1) as i64 * (j + 1) as i64)
                        .collect_vec()
                })
                .collect_vec();
            let instance = Instance { n, cost };

            let timer = Timer::new();
            let _ = hungarian_algorithm(&instance);
            let elapsed = timer.get_elapsed_time();
            eprintln!("test_large: n = {n}, {elapsed} ms");
        }
    }

    #[test]
    fn test_large_product() {
        let n = 100;
        let mut rng = Pcg32::seed_from_u64(0);
        let a = (0..n).map(|_| rng.random_range(1..1000)).collect_vec();
        let b = (0..n).map(|_| rng.random_range(1..1000)).collect_vec();
        let cost = (0..n)
            .map(|i| (0..n).map(|j| a[i] * b[j]).collect_vec())
            .collect_vec();
        let instance = Instance { n, cost };

        let timer = Timer::new();
        let ans = hungarian_algorithm(&instance);
        let elapsed = timer.get_elapsed_time();
        eprintln!("test_large_product: n = {n}, {elapsed} ms");

        let expected = a
            .iter()
            .sorted()
            .zip(b.iter().sorted().rev())
            .map(|(x, y)| x * y)
            .sum::<i64>();
        assert_eq!(ans.cost, expected);
    }

    fn bruteforce(instance: &Instance<i64>) -> i64 {
        assert!(instance.n < 20);
        let n = instance.n;
        let mut dp = vec![i64::MAX; 1 << n];
        let mut dp_prev = vec![i64::MAX; 1 << n];
        dp_prev[0] = 0;
        for u in 0..n {
            dp.fill(i64::MAX);
            for s in 0..1 << n {
                if dp_prev[s] == i64::MAX {
                    continue;
                }
                for v in 0..n {
                    let new_s = s | 1 << v;
                    if s == new_s {
                        continue;
                    }
                    let new_cost = dp_prev[s] + instance.cost[u][v];
                    dp[new_s] = dp[new_s].min(new_cost);
                }
            }

            std::mem::swap(&mut dp, &mut dp_prev);
        }

        dp_prev.iter().copied().min().unwrap()
    }
}
