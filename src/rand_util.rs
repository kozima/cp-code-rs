mod rand_util {
    pub fn gen_random_permutation(n: usize, seed: u64) -> Vec<usize> {
        use rand_pcg::Pcg32;
        use rand::*;

        let mut perm = (0..n).collect::<Vec<_>>();
        let mut rng = Pcg32::seed_from_u64(seed);
        for i in 1..n {
            let j = rng.gen_range(0..i);
            perm.swap(i, j);
        }
        perm
    }

    pub fn gen_random_shallow_tree(n: usize, seed: u64) -> Vec<(usize, usize)> {
        use rand_pcg::Pcg32;
        use rand::*;

        let perm = gen_random_permutation(n, seed ^ 1);
        let mut rng = Pcg32::seed_from_u64(seed);
        let mut es = vec![];
        for i in 1..n {
            let j = rng.gen_range(0..i);
            es.push((perm[j], perm[i]));
        }

        es
    }
}
