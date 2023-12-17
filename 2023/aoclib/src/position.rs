use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn left(&self) -> Option<Self> {
        if self.col > 0 {
            Some(Self {
                row: self.row,
                col: self.col - 1,
            })
        } else {
            None
        }
    }

    pub fn right(&self) -> Option<Self> {
        Some(Self {
            row: self.row,
            col: self.col + 1,
        })
    }

    pub fn above(&self) -> Option<Self> {
        if self.row > 0 {
            Some(Self {
                row: self.row - 1,
                col: self.col,
            })
        } else {
            None
        }
    }

    pub fn below(&self) -> Option<Self> {
        Some(Self {
            row: self.row + 1,
            col: self.col,
        })
    }

    pub fn in_direction(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Up => self.above(),
            Direction::Down => self.below(),
        }
    }
}
