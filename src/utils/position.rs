use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Position {
    pub r: i32,
    pub c: i32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.c, self.r)
    }
}

impl Position {
    pub const fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }

    pub fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                r: self.r - 1,
                c: self.c,
            },
            Direction::UpRight => Self {
                r: self.r - 1,
                c: self.c + 1,
            },
            Direction::Right => Self {
                r: self.r,
                c: self.c + 1,
            },
            Direction::DownRight => Self {
                r: self.r + 1,
                c: self.c + 1,
            },
            Direction::Down => Self {
                r: self.r + 1,
                c: self.c,
            },
            Direction::DownLeft => Self {
                r: self.r + 1,
                c: self.c - 1,
            },
            Direction::Left => Self {
                r: self.r,
                c: self.c - 1,
            },
            Direction::UpLeft => Self {
                r: self.r - 1,
                c: self.c - 1,
            },
        }
    }

    pub const fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            c: self.c + other.c,
        }
    }

    pub const fn times(self, n: i32) -> Self {
        Self {
            r: self.r * n,
            c: self.c * n,
        }
    }

    pub const fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            c: self.c - other.c,
        }
    }
}

pub const ORIGIN: Position = Position { r: 0, c: 0 };

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::UpRight => Direction::DownRight,
            Direction::Right => Direction::Down,
            Direction::DownRight => Direction::DownLeft,
            Direction::Down => Direction::Left,
            Direction::DownLeft => Direction::UpLeft,
            Direction::Left => Direction::Up,
            Direction::UpLeft => Direction::UpRight,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::UpRight => Direction::UpLeft,
            Direction::Right => Direction::Up,
            Direction::DownRight => Direction::UpRight,
            Direction::Down => Direction::Right,
            Direction::DownLeft => Direction::DownRight,
            Direction::Left => Direction::Down,
            Direction::UpLeft => Direction::DownLeft,
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Right => Direction::Left,
            Direction::DownRight => Direction::UpLeft,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRight,
            Direction::Left => Direction::Right,
            Direction::UpLeft => Direction::DownRight,
        }
    }
}

pub const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

pub const ORTHOGONAL: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];
