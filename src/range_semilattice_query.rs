pub trait Semilattice {
    type S: Clone;
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct RangeSemilatticeQuery<T: Semilattice> {
    table: Vec<Vec<T::S>>,
}

impl<T: Semilattice> RangeSemilatticeQuery<T>
where
    T::S: Clone,
{
    pub fn new(a: &[T::S]) -> Self {
        let n = a.len();
        let mut table: Vec<Vec<_>> = vec![a.to_vec()];
        for i in 1.. {
            if 1 << i > n {
                break;
            }
            table.push({
                let step = 1 << i - 1;
                (0..=(n - (1 << i)))
                    .map(|j| {
                        T::binary_operation(
                            &table[i - 1][j],
                            &table[i - 1][j + step],
                        )
                    })
                    .collect()
            });
        }
        RangeSemilatticeQuery { table }
    }

    pub fn get(&self, l: usize, r: usize) -> T::S {
        assert!(l < r);
        if l + 1 == r {
            self.table[0][l].clone()
        } else {
            let k = (usize::BITS - 1 - (r - l).leading_zeros()) as usize;
            T::binary_operation(&self.table[k][l], &self.table[k][r - (1 << k)])
        }
    }

    // find r such that
    // 1. pred(&self.get(l..r1)) is true for l < r1 < r
    // 2. either r == n or pred(&self.get(l..=r)) is false
    pub fn partition_point<P>(&self, l: usize, mut pred: P) -> usize
    where
        P: FnMut(&T::S) -> bool,
    {
        let n = self.table[0].len();
        let mut ans = l;
        let mut val = self.table[0][l].clone();
        for i in (0..self.table.len()).rev() {
            let r = ans + (1 << i);
            if r <= n {
                let next_val = T::binary_operation(&val, &self.table[i][ans]);
                if pred(&next_val) {
                    ans = r;
                    val = next_val;
                }
            }
        }

        assert!(ans == l || pred(&self.table[0][ans - 1]));
        assert!(pred(&self.get(l, ans)));
        assert!(ans == n || !pred(&self.get(l, ans + 1)));
        ans
    }
}

#[cfg(test)]
mod test_rmq {
    use rand::*;
    use rand_pcg::Pcg32;

    use crate::range_semilattice_query::*;

    pub struct Max;
    impl Semilattice for Max {
        type S = usize;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }

    fn gen_seq(n: usize, ub: usize, rng: &mut Pcg32) -> Vec<usize> {
        (0..n).map(|_| rng.random_range(0..=ub)).collect()
    }

    #[test]
    fn test_rmq() {
        let mut rng = Pcg32::seed_from_u64(0);
        for _ in 0..100 {
            let n = 100;
            let a = gen_seq(n, 10000, &mut rng);
            let rmq = RangeSemilatticeQuery::<Max>::new(&a);
            for l in 0..n {
                for r in l..n {
                    assert_eq!(
                        rmq.get(l, r + 1),
                        *a[l..=r].iter().max().unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn test_partition_point() {
        let mut rng = Pcg32::seed_from_u64(0);
        for _ in 0..100 {
            let n = rng.random_range(10..20);
            let a = gen_seq(n, 10000, &mut rng);
            let rmq = RangeSemilatticeQuery::<Max>::new(&a);
            for l in 0..n {
                for r in l..n {
                    let v = *a[l..=r].iter().max().unwrap();
                    let r1 = rmq.partition_point(l, |x| *x <= v);
                    assert!(r <= r1);
                    for i in r..r1 {
                        assert!(a[i] <= v);
                    }
                    assert!(r1 == a.len() || a[r1] > v);
                }
            }
        }
    }

    #[test]
    fn test_single_element() {
        let a = vec![42];
        let rmq = RangeSemilatticeQuery::<Max>::new(&a);
        assert_eq!(rmq.get(0, 1), 42);
    }

    #[test]
    fn test_all_same_values() {
        let a = vec![7; 10];
        let rmq = RangeSemilatticeQuery::<Max>::new(&a);
        for l in 0..a.len() {
            for r in l..a.len() {
                assert_eq!(rmq.get(l, r + 1), 7);
            }
        }
    }

    #[test]
    fn test_empty_array() {
        let a: Vec<usize> = vec![];
        let rmq = RangeSemilatticeQuery::<Max>::new(&a);
        // getの呼び出しはできない（assert失敗するはず）ので、panicしないことだけ確認
        assert_eq!(rmq.table.len(), 1);
        assert_eq!(rmq.table[0].len(), 0);
    }
}
