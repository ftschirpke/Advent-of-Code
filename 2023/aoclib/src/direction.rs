#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn each() -> impl Iterator<Item = Self> {
        [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
        .into_iter()
    }
}
