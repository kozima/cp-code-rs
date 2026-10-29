use std::fmt;

/// Simple timer for measuring elapsed time in milliseconds
pub struct Timer {
    start: std::time::SystemTime,
}

impl Timer {
    pub fn new() -> Timer {
        let start = std::time::SystemTime::now();
        Timer { start }
    }

    pub fn get_elapsed_time(&self) -> u64 {
        let dur = std::time::SystemTime::now()
            .duration_since(self.start)
            .unwrap();
        dur.as_millis() as u64
    }
}

/// 2D position
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn adj(&self, d: Dir) -> Pos {
        match d {
            Dir::L => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Dir::R => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Dir::U => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Dir::D => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

/// 4-directional enum
#[derive(Clone, Copy, Debug)]
pub enum Dir {
    L,
    R,
    U,
    D,
}

pub const ALL_DIRS: [Dir; 4] = [Dir::L, Dir::R, Dir::U, Dir::D];

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Dir::L => 'L',
            Dir::R => 'R',
            Dir::U => 'U',
            Dir::D => 'D',
        };
        write!(f, "{}", c)
    }
}
