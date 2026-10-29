/// Cartesian tree for range minimum/maximum queries
pub struct CartesianTree {
    pub n: usize,
    pub root: usize,
    pub left: Vec<Option<usize>>,
    pub right: Vec<Option<usize>>,
}

impl CartesianTree {
    pub fn new<T: Ord>(a: &[T]) -> Self {
        let n = a.len();
        let mut root = 0;
        let mut stack = vec![0];
        let mut left = vec![None; n];
        let mut right = vec![None; n];
        for i in 1..a.len() {
            while let Some(&j) = stack.last() {
                if a[j] <= a[i] {
                    break;
                }
                stack.pop();
                left[i] = Some(j);
            }
            if let Some(&j) = stack.last() {
                right[j] = Some(i);
            } else {
                root = i;
            }
            stack.push(i);
        }

        CartesianTree {
            n,
            root,
            left,
            right,
        }
    }
}
