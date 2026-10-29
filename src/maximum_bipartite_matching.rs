pub fn maximum_bipartite_matching(
    m: usize,
    n: usize,
    edges: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    use ac_library::maxflow::*;
    let src = m + n;
    let sink = m + n + 1;
    let mut g = MfGraph::new(m + n + 2);
    for i in 0..m {
        g.add_edge(src, i, 1);
    }
    for j in 0..n {
        g.add_edge(m + j, sink, 1);
    }
    for &(i, j) in edges {
        g.add_edge(i, m + j, 1);
    }
    let _ = g.flow(src, sink);
    let mut es = g.edges();
    es.retain(|e| e.flow == 1 && e.from < m && m <= e.to && e.to < m + n);
    es.iter().map(|e| (e.from, e.to - m)).collect()
}

#[cfg(test)]
mod test {
    // use itertools::Itertools;
    // use rand::*;
    // use rand_pcg::Pcg32;

    // struct Timer {
    //     start: std::time::SystemTime,
    // }

    // impl Timer {
    //     fn new() -> Timer {
    //         let start = std::time::SystemTime::now();
    //         Timer { start }
    //     }

    //     fn get_elapsed_time(&self) -> u64 {
    //         let dur = std::time::SystemTime::now()
    //             .duration_since(self.start)
    //             .unwrap();
    //         dur.as_millis() as u64
    //     }
    // }
}
