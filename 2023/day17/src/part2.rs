use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use aoclib::direction::Direction;
use aoclib::{grid::Grid, position::Position, AocError};

use crate::part1::{element_parser, Status, U8Digit};

pub fn process(input: &'static str) -> Result<u32, AocError> {
    let grid: Grid<U8Digit> = Grid::parse_from(input, element_parser)?;
    let mut seen_statuses: HashSet<Status> = HashSet::new();
    let mut to_check: BinaryHeap<Reverse<(u32, Status)>> = BinaryHeap::new();
    let start_statuses = [
        Status {
            pos: Position { row: 0, col: 0 },
            direction: Direction::Down,
            steps_in_direction_taken: 0,
        },
        Status {
            pos: Position { row: 0, col: 0 },
            direction: Direction::Down,
            steps_in_direction_taken: 0,
        },
    ];
    to_check.extend(
        start_statuses
            .into_iter()
            .map(|status| Reverse((0, status))),
    );
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
        if status.pos == end_position && status.steps_in_direction_taken >= 4 {
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
            if status.steps_in_direction_taken == 10 && keep_direction
                || !keep_direction && status.steps_in_direction_taken < 4
            {
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
