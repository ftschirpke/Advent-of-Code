use aoclib::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl Location {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        .iter()
        .copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    StartingPosition,
    LeftToRight,
    TopToBottom,
    LeftToTop,
    LeftToBottom,
    RightToTop,
    RightToBottom,
    None,
}

impl Pipe {
    pub fn connects(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Left => self.connects_left(),
            Direction::Right => self.connects_right(),
            Direction::Up => self.connects_top(),
            Direction::Down => self.connects_bottom(),
        }
    }

    pub fn connects_left(&self) -> bool {
        match self {
            Pipe::StartingPosition => true,
            Pipe::LeftToRight => true,
            Pipe::TopToBottom => false,
            Pipe::LeftToTop => true,
            Pipe::LeftToBottom => true,
            Pipe::RightToTop => false,
            Pipe::RightToBottom => false,
            Pipe::None => false,
        }
    }

    pub fn connects_right(&self) -> bool {
        match self {
            Pipe::StartingPosition => true,
            Pipe::LeftToRight => true,
            Pipe::TopToBottom => false,
            Pipe::LeftToTop => false,
            Pipe::LeftToBottom => false,
            Pipe::RightToTop => true,
            Pipe::RightToBottom => true,
            Pipe::None => false,
        }
    }

    pub fn connects_top(&self) -> bool {
        match self {
            Pipe::StartingPosition => true,
            Pipe::LeftToRight => false,
            Pipe::TopToBottom => true,
            Pipe::LeftToTop => true,
            Pipe::LeftToBottom => false,
            Pipe::RightToTop => true,
            Pipe::RightToBottom => false,
            Pipe::None => false,
        }
    }

    pub fn connects_bottom(&self) -> bool {
        match self {
            Pipe::StartingPosition => true,
            Pipe::LeftToRight => false,
            Pipe::TopToBottom => true,
            Pipe::LeftToTop => false,
            Pipe::LeftToBottom => true,
            Pipe::RightToTop => false,
            Pipe::RightToBottom => true,
            Pipe::None => false,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub starting_location: Location,
    grid: Vec<Vec<Pipe>>,
}

impl Grid {
    pub fn new(grid: Vec<Vec<Pipe>>) -> Result<Self, AocError> {
        let rows = grid.len();
        let cols = grid[0].len();
        if grid.iter().any(|row| row.len() != cols) {
            return Err(AocError::LogicError(
                "Grid rows have different lengths".to_string(),
            ));
        }
        let starting_locations: Vec<_> = grid
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter().enumerate().filter_map(move |(col_idx, &pipe)| {
                    if pipe == Pipe::StartingPosition {
                        Some(Location::new(row_idx, col_idx))
                    } else {
                        None
                    }
                })
            })
            .collect();
        match starting_locations[..] {
            [] => Err(AocError::LogicError(
                "No starting position found".to_string(),
            )),
            [starting_location] => Ok(Self {
                rows,
                cols,
                starting_location,
                grid,
            }),
            _ => Err(AocError::LogicError(
                "Multiple starting positions found".to_string(),
            )),
        }
    }

    pub fn get(&self, location: &Location) -> Option<Pipe> {
        if location.row >= self.rows || location.col >= self.cols {
            return None;
        }
        Some(self.grid[location.row][location.col])
    }

    pub fn get_above(&self, location: Location) -> Option<Pipe> {
        if location.row == 0 {
            return None;
        }
        Some(self.grid[location.row - 1][location.col])
    }
}
