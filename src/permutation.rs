use num_integer::gcd;

#[derive(Debug)]
pub struct Permutation {
    n: usize,
    p: Vec<usize>,
    cycle_index: Vec<Option<usize>>,
    cycles: Vec<Vec<usize>>,
}

impl Permutation {
    pub fn new(p: Vec<usize>) -> Permutation {
        let n = p.len();
        let mut cycles = Vec::new();
        let mut done = vec![false; n];
        let mut cycle_index = vec![None; n];
        for i in 0..n {
            if !done[i] {
                done[i] = true;
                if p[i] != i {
                    let mut cycle = vec![i];
                    while p[*cycle.last().unwrap()] != i {
                        let tmp = p[*cycle.last().unwrap()];
                        cycle.push(tmp);
                    }
                    for &j in &cycle {
                        done[j] = true;
                        cycle_index[j] = Some(cycles.len());
                    }
                    cycles.push(cycle);
                }
            }
        }

        Permutation {
            n,
            p,
            cycle_index,
            cycles,
        }
    }

    pub fn get(&self, i: usize) -> usize {
        self.p[i]
    }

    pub fn get_cycle(&self, i: usize) -> Option<&Vec<usize>> {
        self.cycle_index[i].map(|j| &self.cycles[j])
    }

    pub fn repeat(&self, i: usize) -> Permutation {
        let mut cycles = Vec::new();
        for cycle in &self.cycles {
            let len = cycle.len();
            if i % len == 0 {
                continue;
            }
            let g = gcd(len as i64, i as i64) as usize;
            for mut j in 0..g {
                let mut c = vec![cycle[j]];
                for _ in 1..len / g {
                    j = (j + i) % len;
                    c.push(cycle[j]);
                }
                assert!(c.len() > 1);
                cycles.push(c);
            }
        }

        let mut p: Vec<_> = (0..self.n).collect();
        let mut cycle_index = vec![None; self.n];
        for (j, c) in cycles.iter().enumerate() {
            for i in 0..c.len() {
                cycle_index[c[i]] = Some(j);
                if i == 0 {
                    p[*c.last().unwrap()] = c[0];
                } else {
                    p[c[i - 1]] = c[i];
                }
            }
        }

        Permutation {
            n: self.n,
            p,
            cycle_index,
            cycles,
        }
    }

    pub fn apply<T: Clone + Copy>(p: &Permutation, a: Vec<T>) -> Vec<T> {
        p.p.iter().map(|i| a[*i]).collect()
    }
}
