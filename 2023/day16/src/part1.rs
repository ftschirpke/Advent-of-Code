use std::collections::VecDeque;

use aoclib::AocError;
use nom::{
    branch::alt,
    character::complete::{char as char_parser, line_ending},
    combinator::value,
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeamRedirection {
    SlashMirror,
    BackslashMirror,
    HorizontalSplitter,
    VerticalSplitter,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row_idx: usize,
    pub col_idx: usize,
}

impl Position {
    pub fn next(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Right => Some(Position {
                row_idx: self.row_idx,
                col_idx: self.col_idx + 1,
            }),
            Direction::Left => {
                if self.col_idx == 0 {
                    None
                } else {
                    Some(Position {
                        row_idx: self.row_idx,
                        col_idx: self.col_idx - 1,
                    })
                }
            }
            Direction::Up => {
                if self.row_idx == 0 {
                    None
                } else {
                    Some(Position {
                        row_idx: self.row_idx - 1,
                        col_idx: self.col_idx,
                    })
                }
            }
            Direction::Down => Some(Position {
                row_idx: self.row_idx + 1,
                col_idx: self.col_idx,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub row_num: usize,
    pub col_num: usize,
    rows: Vec<Vec<BeamRedirection>>,
}

impl Grid {
    pub fn new(rows: Vec<Vec<BeamRedirection>>) -> Result<Self, AocError> {
        let row_num = rows.len();
        if row_num == 0 {
            Ok(Self {
                row_num,
                col_num: 0,
                rows,
            })
        } else {
            let col_num = rows[0].len();
            if rows.iter().any(|row| row.len() != col_num) {
                Err(AocError::LogicError(
                    "Not all rows have the same length".to_string(),
                ))
            } else {
                Ok(Self {
                    row_num,
                    col_num,
                    rows,
                })
            }
        }
    }

    pub fn get(&self, pos: &Position) -> Option<BeamRedirection> {
        if pos.row_idx >= self.row_num || pos.col_idx >= self.col_num {
            None
        } else {
            Some(self.rows[pos.row_idx][pos.col_idx])
        }
    }
}

impl BeamRedirection {
    pub fn redirect_from(&self, dir: Direction) -> (Direction, Option<Direction>) {
        match self {
            Self::SlashMirror => match dir {
                Direction::Right => (Direction::Up, None),
                Direction::Left => (Direction::Down, None),
                Direction::Up => (Direction::Right, None),
                Direction::Down => (Direction::Left, None),
            },
            Self::BackslashMirror => match dir {
                Direction::Right => (Direction::Down, None),
                Direction::Left => (Direction::Up, None),
                Direction::Up => (Direction::Left, None),
                Direction::Down => (Direction::Right, None),
            },
            Self::HorizontalSplitter => match dir {
                Direction::Right => (Direction::Right, None),
                Direction::Left => (Direction::Left, None),
                Direction::Up | Direction::Down => (Direction::Left, Some(Direction::Right)),
            },
            Self::VerticalSplitter => match dir {
                Direction::Right | Direction::Left => (Direction::Up, Some(Direction::Down)),
                Direction::Up => (Direction::Up, None),
                Direction::Down => (Direction::Down, None),
            },
            Self::None => (dir, None),
        }
    }
}

fn parse_redirection(input: &str) -> IResult<&str, BeamRedirection> {
    alt((
        value(BeamRedirection::None, char_parser('.')),
        value(BeamRedirection::SlashMirror, char_parser('/')),
        value(BeamRedirection::BackslashMirror, char_parser('\\')),
        value(BeamRedirection::HorizontalSplitter, char_parser('-')),
        value(BeamRedirection::VerticalSplitter, char_parser('|')),
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<BeamRedirection>> {
    terminated(many1(parse_redirection), line_ending)(input)
}

pub fn parse_grid(input: &str) -> Result<Grid, AocError> {
    let (rest, grid) = many1(parse_line)(input)?;
    if rest.is_empty() {
        Grid::new(grid)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing the grid left rest: {}",
            rest
        )))
    }
}

pub fn amount_of_energized_tiles(grid: &Grid, start: (Position, Direction)) -> usize {
    let mut visited = vec![vec![false; grid.col_num]; grid.row_num];
    let mut to_visit = VecDeque::from([start]);
    let mut redirections_visited: Vec<(Position, Direction)> = Vec::new();
    while !to_visit.is_empty() {
        let (position, direction) = to_visit.pop_front().unwrap();
        let current_redirection = grid.get(&position);
        if let Some(current_redirection) = current_redirection {
            if current_redirection != BeamRedirection::None {
                if redirections_visited.contains(&(position, direction)) {
                    continue;
                } else {
                    redirections_visited.push((position, direction));
                }
            }

            visited[position.row_idx][position.col_idx] = true;
            let (new_direction, optional_direction) = current_redirection.redirect_from(direction);
            if let Some(new_position) = position.next(new_direction) {
                to_visit.push_back((new_position, new_direction));
            }
            if let Some(new_direction) = optional_direction {
                if let Some(new_position) = position.next(new_direction) {
                    to_visit.push_back((new_position, new_direction));
                }
            }
        }
    }

    visited
        .into_iter()
        .flat_map(|visited_row| visited_row.into_iter().filter(|was_visited| *was_visited))
        .count()
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let grid = parse_grid(input)?;
    let start = (
        Position {
            row_idx: 0,
            col_idx: 0,
        },
        Direction::Right,
    );
    Ok(amount_of_energized_tiles(&grid, start))
}
