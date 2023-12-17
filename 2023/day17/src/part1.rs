use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use aoclib::direction::Direction;
use aoclib::{grid::Grid, position::Position, AocError};
use nom::{character::complete::one_of, IResult};

pub fn element_parser(input: &str) -> IResult<&str, U8Digit> {
    let (input, digit) = one_of("123456789")(input)?;
    let val = digit.to_digit(10).unwrap() as u8;
    let digit = U8Digit { val };
    Ok((input, digit))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct U8Digit {
    pub val: u8,
}

impl From<&U8Digit> for char {
    fn from(value: &U8Digit) -> Self {
        value.val.to_string().chars().last().unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Status {
    pub pos: Position,
    pub direction: Direction,
    pub steps_in_direction_taken: u8,
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_abs = self.pos.row + self.pos.col;
        let other_abs = other.pos.row + other.pos.col;
        if self_abs == other_abs {
            self.steps_in_direction_taken
                .cmp(&other.steps_in_direction_taken)
        } else {
            self_abs.cmp(&other_abs)
        }
    }
}

pub fn process(input: &'static str) -> Result<u32, AocError> {
    let grid: Grid<U8Digit> = Grid::parse_from(input, element_parser)?;
    let mut seen_statuses: HashSet<Status> = HashSet::new();
    let mut to_check: BinaryHeap<Reverse<(u32, Status)>> = BinaryHeap::new();
    let start_status = Status {
        pos: Position { row: 0, col: 0 },
        direction: Direction::Right,
        steps_in_direction_taken: 0,
    };
    to_check.push(Reverse((0, start_status)));
    let end_position = Position {
        row: grid.rows - 1,
        col: grid.cols - 1,
    };
    let mut count = 0;
    loop {
        let (heat_loss, status) = to_check.pop().unwrap().0;
        if count > 1_000_000 {
            return Err(AocError::LogicError(
                "Too many iterations, probably infinite loop".to_string(),
            ));
        }
        if status.pos == end_position {
            return Ok(heat_loss);
        }

        if seen_statuses.contains(&status) {
            continue;
        }
        seen_statuses.insert(status);

        let possible_next_statuses = Direction::each().filter_map(|direction| {
            if direction == status.direction.opposite() {
                return None; // cannot turn 180 degrees
            }
            let keep_direction = direction == status.direction;
            if status.steps_in_direction_taken == 3 && keep_direction {
                return None;
            }
            let steps_in_direction_taken = if keep_direction {
                status.steps_in_direction_taken + 1
            } else {
                1
            };
            let pos = status.pos.in_direction(direction)?;
            if let Some(new_heat) = grid.get_pos(&pos) {
                let new_status = Status {
                    pos,
                    direction,
                    steps_in_direction_taken,
                };
                Some((heat_loss + new_heat.val as u32, new_status))
            } else {
                None
            }
        });
        to_check.extend(possible_next_statuses.map(Reverse));
        count += 1;
    }
}
