pub struct Dsu {
    par: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Dsu {
        Self {
            par: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        let p = self.par[a];
        if p == a {
            a
        } else {
            let r = self.find(p);
            self.par[a] = r;
            r
        }
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn merge(&mut self, a: usize, b: usize) {
        let x = self.find(a);
        let y = self.find(b);
        if x != y {
            if self.rank[x] < self.rank[y] {
                self.par[x] = y;
                self.size[y] += self.size[x];
            } else {
                self.par[y] = x;
                self.size[x] += self.size[y];
                if self.rank[x] == self.rank[y] {
                    self.rank[x] += 1
                }
            }
        }
    }

    pub fn size(&mut self, a: usize) -> usize {
        let r = self.find(a);
        self.size[r]
    }
}

// union-find with undo operation

struct UndoInfo {
    root: usize,
    new_child: usize,
    orig_rank: usize,
}

pub struct UndoableDsu {
    par: Vec<usize>,
    rank: Vec<usize>,
    undo_stack: Vec<UndoInfo>,
}

impl UndoableDsu {
    pub fn new(n: usize) -> UndoableDsu {
        Self {
            par: (0..n).collect(),
            rank: vec![0; n],
            undo_stack: Vec::new(),
        }
    }

    pub fn find(&self, a: usize) -> usize {
        let p = self.par[a];
        if p == a { a } else { self.find(p) }
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn merge(&mut self, a: usize, b: usize) {
        let x = self.find(a);
        let y = self.find(b);
        if x != y {
            if self.rank[x] < self.rank[y] {
                self.undo_stack.push(UndoInfo {
                    root: y,
                    new_child: x,
                    orig_rank: self.rank[y],
                });
                self.par[x] = y;
            } else {
                self.undo_stack.push(UndoInfo {
                    root: x,
                    new_child: y,
                    orig_rank: self.rank[x],
                });
                self.par[y] = x;
                if self.rank[x] == self.rank[y] {
                    self.rank[x] += 1
                }
            }
        }
    }

    pub fn undo(&mut self) {
        assert!(!self.undo_stack.is_empty());
        let UndoInfo {
            root,
            new_child,
            orig_rank,
        } = self.undo_stack.pop().unwrap();
        self.par[new_child] = new_child;
        self.rank[root] = orig_rank;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undoable_dsu_basic_undo() {
        let mut dsu = UndoableDsu::new(3);

        // Initial: all nodes separate
        assert!(!dsu.same(0, 1));
        assert!(!dsu.same(1, 2));

        // Merge 0 and 1
        dsu.merge(0, 1);
        assert!(dsu.same(0, 1));

        // Undo merge
        dsu.undo();
        assert!(!dsu.same(0, 1));
    }

    #[test]
    fn test_undoable_dsu_rank_restoration() {
        let mut dsu = UndoableDsu::new(2);

        // Initial state: both nodes have rank 0
        assert_eq!(dsu.rank[0], 0);
        assert_eq!(dsu.rank[1], 0);

        // Merge equal ranks: should increment rank of root
        dsu.merge(0, 1);
        assert_eq!(dsu.rank[0], 1); // Rank incremented
        assert_eq!(dsu.par[1], 0); // 1 is child of 0

        // Undo: rank should be restored to 0
        dsu.undo();
        assert_eq!(dsu.rank[0], 0); // ✓ Rank correctly restored
        assert_eq!(dsu.par[1], 1); // 1 is independent again
    }

    #[test]
    fn test_undoable_dsu_no_rank_change_undo() {
        let mut dsu = UndoableDsu::new(3);

        // Set up: 0 has rank 1, 1 has rank 0
        dsu.merge(0, 2); // Makes rank[0] = 1
        dsu.undo(); // Undo to clean state
        dsu.merge(0, 2); // Again: rank[0] = 1

        // Now merge with different ranks: rank[0] > rank[1]
        // No rank increment should occur
        dsu.merge(0, 1);
        assert_eq!(dsu.rank[0], 1); // Rank unchanged

        // Undo
        dsu.undo();
        assert_eq!(dsu.rank[0], 1); // Rank preserved
        assert_eq!(dsu.par[1], 1); // 1 restored
    }

    #[test]
    fn test_undoable_dsu_multiple_merges_and_undos() {
        let mut dsu = UndoableDsu::new(4);

        // Perform merges
        dsu.merge(0, 1);
        dsu.merge(2, 3);

        // Both pairs should be connected
        assert!(dsu.same(0, 1));
        assert!(dsu.same(2, 3));
        assert!(!dsu.same(0, 2));

        // Undo second merge
        dsu.undo();
        assert!(dsu.same(0, 1));
        assert!(!dsu.same(2, 3)); // ✓ Second merge undone

        // Undo first merge
        dsu.undo();
        assert!(!dsu.same(0, 1)); // ✓ Both undone
        assert!(!dsu.same(2, 3));
    }

    #[test]
    fn test_undoable_dsu_sequence_three_merges() {
        let mut dsu = UndoableDsu::new(4);

        // Merge sequence: (0,1), (1,2), (2,3)
        dsu.merge(0, 1);
        assert_eq!(dsu.undo_stack.len(), 1);

        dsu.merge(1, 2);
        assert_eq!(dsu.undo_stack.len(), 2);

        dsu.merge(2, 3);
        assert_eq!(dsu.undo_stack.len(), 3);

        // All should be connected
        assert!(dsu.same(0, 3));

        // Undo in reverse order
        dsu.undo();
        assert!(!dsu.same(0, 3));

        dsu.undo();
        assert!(!dsu.same(1, 2));

        dsu.undo();
        assert!(!dsu.same(0, 1));
    }

    #[test]
    fn test_undoable_dsu_rank_with_path_compression() {
        let mut dsu = UndoableDsu::new(4);

        // Create a chain: 0-1-2-3
        dsu.merge(0, 1);
        dsu.merge(1, 2);
        dsu.merge(2, 3);

        // All connected
        assert!(dsu.same(0, 3));

        // Undo all
        dsu.undo();
        dsu.undo();
        dsu.undo();

        // All should be independent
        assert!(!dsu.same(0, 1));
        assert!(!dsu.same(1, 2));
        assert!(!dsu.same(2, 3));
    }

    #[test]
    fn test_undoable_dsu_equal_rank_increment_tracking() {
        let mut dsu = UndoableDsu::new(5);

        // Create two trees with rank 1
        dsu.merge(0, 1); // rank[0] = 1
        dsu.undo();
        dsu.merge(0, 1); // rank[0] = 1 again

        dsu.merge(2, 3); // rank[2] = 1
        dsu.undo();
        dsu.merge(2, 3); // rank[2] = 1 again

        // Merge two rank-1 trees
        dsu.merge(0, 2); // Should increment one of them
        let root = dsu.find(0);
        assert_eq!(dsu.rank[root], 2); // Rank becomes 2

        // Undo should restore to rank 1
        dsu.undo();
        assert_eq!(dsu.rank[root], 1); // ✓ Rank correctly restored
    }

    #[test]
    fn test_dsu_basic_merge() {
        let mut dsu = Dsu::new(4);

        dsu.merge(0, 1);
        assert!(dsu.same(0, 1));

        dsu.merge(2, 3);
        assert!(dsu.same(2, 3));

        assert!(!dsu.same(0, 2));
    }

    #[test]
    fn test_dsu_size() {
        let mut dsu = Dsu::new(4);

        assert_eq!(dsu.size(0), 1);
        dsu.merge(0, 1);
        assert_eq!(dsu.size(0), 2);

        dsu.merge(2, 3);
        assert_eq!(dsu.size(2), 2);

        dsu.merge(0, 2);
        assert_eq!(dsu.size(0), 4);
    }
}
