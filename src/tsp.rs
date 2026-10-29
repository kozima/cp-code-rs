// WIP

// closed/open/start-fixed/end-fixed/start-end-fixed
// symmetric/asymmetric
// complete/general

// symmetric
// asymmetric
// metric (triangular inequality)
// euclidean

// ## 1. コスト・距離のバリエーション

// * **対称TSP (Symmetric TSP, STSP)**
//   都市間距離が対称（$d(i,j) = d(j,i)$）。
// * **非対称TSP (Asymmetric TSP, ATSP)**
//   都市間距離が非対称（交通規則や風向きなど）。
// * **メトリックTSP (Metric TSP)**
//   三角不等式を満たす距離を仮定。近似アルゴリズムがよく研究されている。
// * **ユークリッドTSP (Euclidean TSP)**
//   都市を平面上の点としてユークリッド距離を使う。

// ## 2. 部分集合や制約を加えたもの

// * **部分TSP (Subset TSP)**
//   与えられた部分集合だけを訪問すればよい。
// * **プリオリティ付きTSP (Priority TSP)**
//   高優先度の都市を早めに訪問する制約。
// * **時間窓付きTSP (TSP with Time Windows, TSPTW)**
//   各都市に訪問可能な時間帯が指定される。
// * **容量制約付きTSP (Capacitated TSP, CTSP)**
//   輸送・配達を考慮し、車両容量を制約とする。

// ## 3. 動的・確率的なもの

// * **動的TSP (Dynamic TSP)**
//   都市集合や距離が時間とともに変化する。
// * **確率的TSP (Probabilistic TSP)**
//   都市が訪問対象になるかどうかが確率的に決まる。
// * **オンラインTSP**
//   目的地が逐次的に明らかになる。

// ## 4. グラフ構造に基づくもの

// * **グラフTSP (Graph-TSP)**
//   任意の重み付きグラフで定義。
// * **ハミルトン路問題 (Hamiltonian Path Problem)**
//   始点と終点が固定される場合（巡回ではない）。
// * **Steiner TSP**
//   訪問すべき都市以外の頂点も経由可能。

// ## 5. 応用寄りのバリエーション

// * **多セールスマン問題 (mTSP, Multiple TSP)**
//   複数の経路を同時に求める（配送やチーム活動）。
// * **車両経路問題 (VRP, Vehicle Routing Problem)**
//   TSPを一般化し、複数車両＋容量制約＋時間窓などを含む。
// * **ドローンTSP**
//   ドローンの航続距離やバッテリー交換を考慮する。
// * **配達問題・ピックアップ＆デリバリー問題**
//   物資を積んで降ろす順序制約が入る。

// ## 6. 計算の観点から

// * **近似アルゴリズム対象のTSP**
//   Metric TSP は 1.5-近似アルゴリズム（Christofides）が有名。
// * **分枝限定法や整数計画法で解く標準TSP**
//   Concorde などのソルバで最適化研究が進んでいる。
// * **ヒューリスティック・メタヒューリスティック**
//   * 最近近傍法 (Nearest Neighbor)
//   * 2-opt, 3-opt
//   * 遺伝的アルゴリズム
//   * 焼きなまし法
//   * 蟻コロニー最適化

mod tsp {
    struct Instance {}

    pub struct SolverState {
        input: Input,
        cur_ans: Vec<usize>,
        best_ans: Vec<usize>,
        pub cur_score: f64,
        pub best_score: f64,
        pub iteration_count: u64,
        rng: rand_chacha::ChaCha20Rng,
    }

    pub struct Config {
        pub use_2opt: bool,
        pub use_3opt: bool,
        pub use_swap: bool,
        pub use_insert: bool,
        pub use_double_bridge: bool,
        pub use_kick: bool,
        pub use_annealing: bool,
        pub kick_period: u64,
        pub temperature: f64,
    }

    impl Config {
        pub fn new() -> Config {
            Config {
                use_2opt: true,
                use_3opt: true,
                use_swap: false,
                use_insert: false,
                use_double_bridge: false,
                use_kick: true,
                use_annealing: false,
                kick_period: 10_000,
                temperature: 0.0,
            }
        }
    }

    enum Neighbor {
        // (i, j) -> reverse i..j
        TwoOpt(usize, usize),
        // (i, j, k, a, b) -> exchange i..j and j..k, reverse i..j if a, reverse j..k if b
        ThreeOpt(usize, usize, usize, bool, bool),
        // (i, j) -> swap i, j
        Swap(usize, usize),
        // (i, j) -> remove i and insert it in front of j
        Insert(usize, usize),
        // (i, j, k, l) -> swap i..j and k..l
        DoubleBridge(usize, usize, usize, usize),
    }

    #[wasm_bindgen]
    impl SolverState {
        pub fn new(input_str: &str) -> SolverState {
            let input = parse_input(input_str);
            let initial_ans = (0..NUM_CITIES as usize).collect::<Vec<_>>();
            let score = calc_score(&input, &initial_ans);
            let rng = rand_chacha::ChaCha20Rng::seed_from_u64(1);
            SolverState {
                input: input.clone(),
                cur_ans: initial_ans.clone(),
                best_ans: initial_ans.clone(),
                cur_score: score,
                best_score: score,
                iteration_count: 0,
                rng,
            }
        }

        /*
                pub fn visualize(&self, show_best: bool) -> Vis {
                    let input = &self.input;
                    let mut svg = svg::Document::new()
                        .set("viewBox", (-PADDING, -PADDING, WIDTH + PADDING, HEIGHT + PADDING))
                        .set("id", "vis")
                        .set("width", WIDTH + 2 * PADDING)
                        .set("height", HEIGHT + 2 * PADDING);

                    svg = svg.add(
                        svg::node::element::Rectangle::new()
                            .set("x", -PADDING)
                            .set("y", -PADDING)
                            .set("width", WIDTH + 2 * PADDING)
                            .set("height", HEIGHT + 2 * PADDING)
                            .set("fill", "white")
                            .set("stroke-width", "0.0"),
                    );

                    for &(x, y) in input.cities.iter() {
                        let city = svg::node::element::Circle::new()
                            .set("fill", "blue")
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 3);
                        svg = svg.add(city);
                    }

                    svg = svg.add(
                        svg::node::element::Path::new()
                            .set("fill", "none")
                            .set("stroke", "grey")
                            .set("stroke-width", 1)
                            .set("d", svg::node::element::path::Data::new()
                                 .move_to((0, 0))
                                 .line_by((0, WIDTH))
                                 .line_by((HEIGHT, 0))
                                 .line_by((0, -WIDTH))
                                 .close()));

                    if show_best {
                        for (i, j) in self.best_ans.iter().zip(self.best_ans.iter().skip(1).chain(std::iter::once(&self.best_ans[0]))) {
                            let (sx, sy) = input.cities[*i];
                            let (gx, gy) = input.cities[*j];
                            let path = {
                                let data = svg::node::element::path::Data::new()
                                    .move_to((sx, sy))
                                    .line_to((gx, gy));
                                svg::node::element::Path::new()
                                    .set("stroke", "red")
                                    .set("stroke-width", 5)
                                    .set("stroke-opacity", "0.2")
                                    .set("d", data)
                            };
                            svg = svg.add(path);
                        }
                    }

                    for (i, j) in self.cur_ans.iter().zip(self.cur_ans.iter().skip(1).chain(std::iter::once(&self.cur_ans[0]))) {
                        let (sx, sy) = input.cities[*i];
                        let (gx, gy) = input.cities[*j];
                        let path = {
                            let data = svg::node::element::path::Data::new()
                                .move_to((sx, sy))
                                .line_to((gx, gy));
                            svg::node::element::Path::new()
                                .set("stroke", "black")
                                .set("stroke-width", 1)
                                .set("d", data)
                        };
                        svg = svg.add(path);
                    }

                    Vis { svg_doc: svg }
                }

                pub fn output_string(&self) -> String {
                    self.cur_ans.iter().join(" ")
                }
        */

        fn generate_neighbor(&mut self, mask: u32) -> Neighbor {
            assert!(mask > 0);
            let t = {
                let mut m = mask;
                let mut k = 0;
                let mut t = 0;
                for i in 0.. {
                    if m & 1 != 0 {
                        if self.rng.gen_range(0..=k) == 0 {
                            t = i;
                        }
                        k += 1;
                    }
                    m /= 2;
                    if m == 0 {
                        break;
                    }
                }
                assert!(k > 0);
                t
            };
            match t {
                0 => loop {
                    let i = self.rng.gen_range(0..NUM_CITIES as usize);
                    let j = self.rng.gen_range(0..NUM_CITIES as usize);
                    if i + 1 < j {
                        break Neighbor::TwoOpt(i, j);
                    }
                },
                1 => self.generate_threeopt(),
                2 => loop {
                    let i = self.rng.gen_range(0..NUM_CITIES as usize);
                    let j = self.rng.gen_range(0..NUM_CITIES as usize);
                    if i != j {
                        break Neighbor::Swap(i.min(j), i.max(j));
                    }
                },
                3 => loop {
                    let i = self.rng.gen_range(0..NUM_CITIES as usize);
                    let j = self.rng.gen_range(0..NUM_CITIES as usize);
                    if i + 1 < j {
                        break Neighbor::Insert(i, j);
                    }
                },
                4 => self.generate_double_bridge(),
                _ => unreachable!(),
            }
        }

        fn generate_threeopt(&mut self) -> Neighbor {
            loop {
                let i = self.rng.gen_range(0..NUM_CITIES as usize);
                let j = self.rng.gen_range(0..NUM_CITIES as usize);
                let k = self.rng.gen_range(0..NUM_CITIES as usize);
                let mut a = [i, j, k];
                a.sort();
                let [i, j, k] = a;
                if i < j && j < k {
                    let a = self.rng.gen_bool(0.5);
                    let b = self.rng.gen_bool(0.5);
                    if !a || !b {
                        break Neighbor::ThreeOpt(i, j, k, a, b);
                    }
                }
            }
        }

        fn generate_double_bridge(&mut self) -> Neighbor {
            loop {
                let i = self.rng.gen_range(0..NUM_CITIES as usize);
                let j = self.rng.gen_range(0..NUM_CITIES as usize);
                let k = self.rng.gen_range(0..NUM_CITIES as usize);
                let l = self.rng.gen_range(0..NUM_CITIES as usize);
                let mut a = [i, j, k, l];
                a.sort();
                let [i, j, k, l] = a;
                if i < j && j < k && k < l {
                    break Neighbor::DoubleBridge(i, j, k, l);
                }
            }
        }

        fn generate_kick_neighbor(&mut self) -> Neighbor {
            self.generate_double_bridge()
        }

        fn move_to_neighbor(&mut self, nbr: &Neighbor, diff: f64) {
            match nbr {
                Neighbor::TwoOpt(i, j) => {
                    self.cur_ans[*i..*j].reverse();
                }
                Neighbor::ThreeOpt(i, j, k, rev_l, rev_r) => {
                    if !rev_l {
                        self.cur_ans[*i..*j].reverse();
                    }
                    if !rev_r {
                        self.cur_ans[*j..*k].reverse();
                    }
                    self.cur_ans[*i..*k].reverse();
                }
                Neighbor::Swap(i, j) => {
                    self.cur_ans.swap(*i, *j);
                }
                Neighbor::Insert(i, j) => {
                    self.cur_ans[i + 1..*j].reverse();
                    self.cur_ans[*i..*j].reverse();
                }
                Neighbor::DoubleBridge(i, j, k, l) => {
                    self.cur_ans[*i..*j].reverse();
                    self.cur_ans[*j..*k].reverse();
                    self.cur_ans[*k..*l].reverse();
                    self.cur_ans[*i..*l].reverse();
                }
            }
            self.cur_score += diff;
            if self.cur_score < self.best_score {
                self.best_score = self.cur_score;
                self.best_ans = self.cur_ans.clone();
            }
        }

        fn evaluate_neighbor(&self, nbr: &Neighbor) -> f64 {
            let n = NUM_CITIES as usize;
            match nbr {
                Neighbor::TwoOpt(i, j) => {
                    assert!(*i + 1 < *j);
                    let a = self.cur_ans[(i + n - 1) % n];
                    let b = self.cur_ans[*i];
                    let c = self.cur_ans[(j + n - 1) % n];
                    let d = self.cur_ans[*j];
                    calc_distance(self.input.cities[a], self.input.cities[c])
                        + calc_distance(
                            self.input.cities[b],
                            self.input.cities[d],
                        )
                        - calc_distance(
                            self.input.cities[a],
                            self.input.cities[b],
                        )
                        - calc_distance(
                            self.input.cities[c],
                            self.input.cities[d],
                        )
                }
                Neighbor::ThreeOpt(i, j, k, rev_l, rev_r) => {
                    assert!(i < j && j < k);
                    let a = self.cur_ans[(i + n - 1) % n];
                    let b = self.cur_ans[*i];
                    let c = self.cur_ans[(j + n - 1) % n];
                    let d = self.cur_ans[*j];
                    let e = self.cur_ans[(k + n - 1) % n];
                    let f = self.cur_ans[*k];
                    let dec =
                    // a -> b ... c -> d ... e -> f
                        calc_distance(self.input.cities[a], self.input.cities[b])
                        + calc_distance(self.input.cities[c], self.input.cities[d])
                        + calc_distance(self.input.cities[e], self.input.cities[f]);
                    let inc = {
                        // a -> p ... q -> r ... s -> f
                        let p = if *rev_r { e } else { d };
                        let q = if *rev_r { d } else { e };
                        let r = if *rev_l { c } else { b };
                        let s = if *rev_l { b } else { c };
                        calc_distance(
                            self.input.cities[a],
                            self.input.cities[p],
                        ) + calc_distance(
                            self.input.cities[q],
                            self.input.cities[r],
                        ) + calc_distance(
                            self.input.cities[s],
                            self.input.cities[f],
                        )
                    };
                    inc - dec
                }
                Neighbor::Swap(i, j) => {
                    assert!(i < j);
                    if *i + 1 == *j {
                        // a b c d -> a c b d
                        let a = self.cur_ans[(i + n - 1) % n];
                        let b = self.cur_ans[*i];
                        let c = self.cur_ans[*j];
                        let d = self.cur_ans[(j + 1) % n];
                        calc_distance(
                            self.input.cities[a],
                            self.input.cities[c],
                        ) + calc_distance(
                            self.input.cities[b],
                            self.input.cities[d],
                        ) - calc_distance(
                            self.input.cities[a],
                            self.input.cities[b],
                        ) - calc_distance(
                            self.input.cities[c],
                            self.input.cities[d],
                        )
                    } else if *i == 0 && *j == n - 1 {
                        // a b ... c d -> d b ... c a
                        let a = self.cur_ans[0];
                        let b = self.cur_ans[1];
                        let c = self.cur_ans[n - 2];
                        let d = self.cur_ans[n - 1];
                        calc_distance(
                            self.input.cities[a],
                            self.input.cities[c],
                        ) + calc_distance(
                            self.input.cities[b],
                            self.input.cities[d],
                        ) - calc_distance(
                            self.input.cities[a],
                            self.input.cities[b],
                        ) - calc_distance(
                            self.input.cities[c],
                            self.input.cities[d],
                        )
                    } else {
                        assert!(i + 1 <= j - 1);
                        // a b c ... d e f -> a e c ... d b f
                        let a = self.cur_ans[(i + n - 1) % n];
                        let b = self.cur_ans[*i];
                        let c = self.cur_ans[i + 1];
                        let d = self.cur_ans[j - 1];
                        let e = self.cur_ans[*j];
                        let f = self.cur_ans[(j + 1) % n];
                        calc_distance(
                            self.input.cities[a],
                            self.input.cities[e],
                        ) + calc_distance(
                            self.input.cities[e],
                            self.input.cities[c],
                        ) + calc_distance(
                            self.input.cities[d],
                            self.input.cities[b],
                        ) + calc_distance(
                            self.input.cities[b],
                            self.input.cities[f],
                        ) - calc_distance(
                            self.input.cities[a],
                            self.input.cities[b],
                        ) - calc_distance(
                            self.input.cities[b],
                            self.input.cities[c],
                        ) - calc_distance(
                            self.input.cities[d],
                            self.input.cities[e],
                        ) - calc_distance(
                            self.input.cities[e],
                            self.input.cities[f],
                        )
                    }
                }
                Neighbor::Insert(i, j) => {
                    assert!(*i + 1 < *j);
                    // a b c ... d e -> a c ... d b e
                    let a = self.cur_ans[(i + n - 1) % n];
                    let b = self.cur_ans[*i];
                    let c = self.cur_ans[i + 1];
                    let d = self.cur_ans[j - 1];
                    let e = self.cur_ans[*j];
                    calc_distance(self.input.cities[a], self.input.cities[c])
                        + calc_distance(
                            self.input.cities[d],
                            self.input.cities[b],
                        )
                        + calc_distance(
                            self.input.cities[b],
                            self.input.cities[e],
                        )
                        - calc_distance(
                            self.input.cities[a],
                            self.input.cities[b],
                        )
                        - calc_distance(
                            self.input.cities[b],
                            self.input.cities[c],
                        )
                        - calc_distance(
                            self.input.cities[d],
                            self.input.cities[e],
                        )
                }
                Neighbor::DoubleBridge(i, j, k, l) => {
                    let a = self.cur_ans[(i + n - 1) % n];
                    let b = self.cur_ans[*i];
                    let c = self.cur_ans[(j + n - 1) % n];
                    let d = self.cur_ans[*j];
                    let e = self.cur_ans[(k + n - 1) % n];
                    let f = self.cur_ans[*k];
                    let g = self.cur_ans[(l + n - 1) % n];
                    let h = self.cur_ans[*l];
                    calc_distance(self.input.cities[a], self.input.cities[f])
                        + calc_distance(
                            self.input.cities[c],
                            self.input.cities[h],
                        )
                        + calc_distance(
                            self.input.cities[e],
                            self.input.cities[b],
                        )
                        + calc_distance(
                            self.input.cities[g],
                            self.input.cities[d],
                        )
                        - calc_distance(
                            self.input.cities[a],
                            self.input.cities[b],
                        )
                        - calc_distance(
                            self.input.cities[c],
                            self.input.cities[d],
                        )
                        - calc_distance(
                            self.input.cities[e],
                            self.input.cities[f],
                        )
                        - calc_distance(
                            self.input.cities[g],
                            self.input.cities[h],
                        )
                }
            }
        }

        fn accept(&mut self, diff: f64, config: &Config) -> bool {
            if diff <= 0.0 {
                true
            } else if !config.use_annealing {
                false
            } else {
                let temp = config.temperature;
                let prob = (-diff / temp).exp();
                self.rng.gen_bool(prob)
            }
        }

        pub fn kick(&mut self) {
            let nbr = self.generate_kick_neighbor();
            let diff = self.evaluate_neighbor(&nbr);
            self.move_to_neighbor(&nbr, diff);
            assert!(
                (self.cur_score - calc_score(&self.input, &self.cur_ans))
                    < 1e-3
            );
        }

        pub fn improve(&mut self, n: usize, config: &Config) {
            let mut mask = 0;
            if config.use_2opt {
                mask |= 1 << 0;
            }
            if config.use_3opt {
                mask |= 1 << 1;
            }
            if config.use_swap {
                mask |= 1 << 2;
            }
            if config.use_insert {
                mask |= 1 << 3;
            }
            if config.use_double_bridge {
                mask |= 1 << 4;
            }

            if mask > 0 {
                for _ in 0..n {
                    if config.use_kick
                        && (self.iteration_count % config.kick_period as u64)
                            == 0
                    {
                        // kick
                        self.kick();
                    } else {
                        let nbr = self.generate_neighbor(mask);
                        let diff = self.evaluate_neighbor(&nbr);
                        if self.accept(diff, config) {
                            self.move_to_neighbor(&nbr, diff);
                            assert!(
                                (self.cur_score
                                    - calc_score(&self.input, &self.cur_ans))
                                    < 1e-3
                            );
                        }
                    }
                    self.iteration_count += 1;
                }
            }
        }
    }

    // pub fn solve(instance: &Instance, config: &Config) -> Result {

    // }

    pub fn solve<T>(n: usize, distance: &[&[T]], config: &Config) -> Result {}
}
