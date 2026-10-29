#[derive(Debug)]
pub struct WeightedDsu<T> {
    par: Vec<usize>,
    rank: Vec<usize>,
    diff: Vec<T>,
}

impl<T> WeightedDsu<T>
where
    T: Default
        + Clone
        + Copy
        + Eq
        + std::ops::AddAssign
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>,
{
    pub fn new(n: usize) -> WeightedDsu<T> {
        Self {
            par: (0..n).collect(),
            rank: vec![0; n],
            diff: vec![T::default(); n],
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        let p = self.par[a];
        if p == a {
            a
        } else {
            let r = self.find(p);
            self.par[a] = r;
            let d = self.diff[p];
            self.diff[a] += d;
            r
        }
    }

    pub fn add(&mut self, a: usize, b: usize, d: T) -> bool {
        let x = self.find(a);
        let y = self.find(b);
        if x == y {
            self.diff[a] == self.diff[b] + d
        } else if self.rank[x] < self.rank[y] {
            self.par[x] = y;
            self.diff[x] = self.diff[b] - self.diff[a] + d;
            true
        } else {
            self.par[y] = x;
            self.diff[y] = self.diff[a] - self.diff[b] - d;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1
            }
            true
        }
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn diff(&mut self, a: usize, b: usize) -> Option<T> {
        let ra = self.find(a);
        let rb = self.find(b);
        (ra == rb).then(|| self.diff[a] - self.diff[b])
    }
}

#[cfg(test)]
mod test {
    use crate::weighted_dsu::WeightedDsu;
    use itertools::*;
    use rand::*;
    use rand_pcg::Pcg32;

    #[test]
    fn test_hand() {
        let mut dsu = WeightedDsu::new(3);
        for (i, j) in iproduct!(0..3, 0..3) {
            assert_eq!(dsu.same(i, j), i == j);
            assert_eq!(dsu.diff(i, j), (i == j).then_some(0));
        }

        dsu.add(0, 1, 3);
        assert!(dsu.same(0, 1));
        assert!(!dsu.same(0, 2));
        assert!(!dsu.same(1, 2));
        assert_eq!(dsu.diff(0, 1), Some(3));
        assert_eq!(dsu.diff(1, 0), Some(-3));
        assert_eq!(dsu.diff(0, 2), None);
        assert_eq!(dsu.diff(2, 0), None);
        assert_eq!(dsu.diff(1, 2), None);
        assert_eq!(dsu.diff(2, 1), None);

        dsu.add(0, 1, 3);
        dsu.add(0, 2, 5);
        assert!(dsu.same(0, 1));
        assert!(dsu.same(0, 2));
        assert!(dsu.same(1, 2));
        assert_eq!(dsu.diff(0, 1), Some(3));
        assert_eq!(dsu.diff(1, 0), Some(-3));
        assert_eq!(dsu.diff(0, 2), Some(5));
        assert_eq!(dsu.diff(2, 0), Some(-5));
        assert_eq!(dsu.diff(1, 2), Some(2));
        assert_eq!(dsu.diff(2, 1), Some(-2));
    }

    #[test]
    fn test_random_small() {
        let mut rng = Pcg32::seed_from_u64(0);
        for n in 5..20 {
            let a = (0..n).map(|_| rng.random_range(-100..100)).collect_vec();
            let mut dsu = WeightedDsu::new(n);
            for i in 1..n {
                let j = rng.random_range(0..i);
                dsu.add(j, i, a[j] - a[i]);
            }
            for (i, j) in iproduct!(0..n, 0..n) {
                assert_eq!(dsu.diff(i, j), Some(a[i] - a[j]));
            }
        }
    }
}
