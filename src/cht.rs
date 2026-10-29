#[derive(Clone, Copy, Debug)]
pub struct Line {
    // a * x + b * y = c (<=> y = (c - a * x) / b)
    pub a: i128,
    pub b: i128,
    pub c: i128,
}

impl Line {
    pub fn le_at(&self, other: &Line, x: i128) -> bool {
        (self.c - self.a * x) * other.b <= (other.c - other.a * x) * self.b
    }
}

pub struct ConvexHullTrick {
    segments: Vec<(i128, Line)>,
}

impl ConvexHullTrick {
    pub fn new() -> ConvexHullTrick {
        ConvexHullTrick { segments: vec![] }
    }

    pub fn add_line(&mut self, line: Line) {
        while let Some(&(left, prev)) = self.segments.last() {
            if line.le_at(&prev, left) {
                self.segments.pop();
            } else {
                break;
            }
        }

        if let Some(&(
            _,
            Line {
                a: a1,
                b: b1,
                c: c1,
            },
        )) = self.segments.last()
        {
            let Line {
                a: a2,
                b: b2,
                c: c2,
            } = line;
            if a2 * b1 != a1 * b2 {
                let x = (b1 * c2 - b2 * c1 + a2 * b1 - a1 * b2 - 1)
                    / (a2 * b1 - a1 * b2);
                self.segments.push((x, line));
            }
        } else {
            self.segments.push((1, line));
        }
    }

    pub fn get_min(&self, x: i128) -> i128 {
        if self.segments.is_empty() {
            return i128::MAX;
        }

        let i = self.segments.partition_point(|&(left, _)| left <= x);
        let line = self.segments[i].1;
        (line.c - line.a * x) / line.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let line = Line { a: 1, b: 1, c: 5 };
        assert_eq!(line.a, 1);
        assert_eq!(line.b, 1);
        assert_eq!(line.c, 5);
    }

    #[test]
    fn test_line_comparison_at_x() {
        // Line 1: y = (5 - 1*x) / 1 = 5 - x
        let line1 = Line { a: 1, b: 1, c: 5 };
        // Line 2: y = (6 - 2*x) / 1 = 6 - 2x
        let line2 = Line { a: 2, b: 1, c: 6 };

        // At x=0: line1 = 5, line2 = 6 (line1 <= line2)
        assert!(line1.le_at(&line2, 0));
        // At x=2: line1 = 3, line2 = 2 (line1 > line2)
        assert!(!line1.le_at(&line2, 2));
    }

    #[test]
    fn test_line_with_negative_coefficients() {
        // Line 1: y = (-2 - (-1)*x) / 1 = -2 + x
        let line1 = Line { a: -1, b: 1, c: -2 };
        // Line 2: y = (1 - 0*x) / 1 = 1
        let line2 = Line { a: 0, b: 1, c: 1 };

        // At x=0: line1 = -2, line2 = 1
        assert!(line1.le_at(&line2, 0));
        // At x=5: line1 = 3, line2 = 1
        assert!(!line1.le_at(&line2, 5));
    }

    #[test]
    fn test_cht_empty_creation() {
        let cht = ConvexHullTrick::new();
        assert_eq!(cht.segments.len(), 0);
    }

    #[test]
    fn test_cht_add_single_line() {
        let mut cht = ConvexHullTrick::new();
        let line = Line { a: 1, b: 1, c: 5 };
        cht.add_line(line);
        assert_eq!(cht.segments.len(), 1);
    }

    #[test]
    fn test_cht_add_multiple_lines() {
        let mut cht = ConvexHullTrick::new();

        // Add line 1: y = 1*x + 0
        let line1 = Line { a: -1, b: 1, c: 0 };
        cht.add_line(line1);
        assert_eq!(cht.segments.len(), 1);

        // Add line 2: y = 2*x + 0 (different slope)
        let line2 = Line { a: -2, b: 1, c: 0 };
        cht.add_line(line2);
        assert_eq!(cht.segments.len(), 2);
    }

    #[test]
    fn test_cht_maintains_convex_hull() {
        let mut cht = ConvexHullTrick::new();

        // Add lines that should form a convex hull
        // Line: y = -1*x + 10 (slope 1, intercept 10)
        let line1 = Line { a: 1, b: 1, c: 10 };
        cht.add_line(line1);

        // Line: y = -2*x + 20 (slope 2, intercept 20)
        let line2 = Line { a: 2, b: 1, c: 20 };
        cht.add_line(line2);

        // Line: y = -3*x + 30 (slope 3, intercept 30)
        let line3 = Line { a: 3, b: 1, c: 30 };
        cht.add_line(line3);

        // The convex hull should contain at least these segments
        assert!(cht.segments.len() > 0);
    }

    #[test]
    fn test_cht_with_horizontal_lines() {
        let mut cht = ConvexHullTrick::new();

        // Horizontal line: y = 5 (slope 0)
        let line1 = Line { a: 0, b: 1, c: 5 };
        cht.add_line(line1);

        // Another line: y = -1*x + 10
        let line2 = Line { a: 1, b: 1, c: 10 };
        cht.add_line(line2);

        assert!(cht.segments.len() >= 1);
    }

    #[test]
    fn test_cht_dp_use_case() {
        // Simulating a simple DP optimization:
        // dp[i] = min(dp[j] + a[i] * b[j] + c[j]) for j < i
        // where lines are represented as: y = b[j] * x + (c[j])
        // with slope b[j] and intercept c[j]

        let mut cht = ConvexHullTrick::new();

        // Simulate DP states with lines
        // Line representing j=0: y = 2*x + 3
        let line0 = Line { a: -2, b: 1, c: 3 };
        cht.add_line(line0);

        // Line representing j=1: y = 3*x + 1
        let line1 = Line { a: -3, b: 1, c: 1 };
        cht.add_line(line1);

        // Line representing j=2: y = 1*x + 8
        let line2 = Line { a: -1, b: 1, c: 8 };
        cht.add_line(line2);

        // Verify structure is maintained
        assert!(cht.segments.len() > 0);
        // Segments should be ordered by x value
        for i in 1..cht.segments.len() {
            let (x_prev, _) = cht.segments[i - 1];
            let (x_curr, _) = cht.segments[i];
            assert!(x_prev < x_curr);
        }
    }
}
