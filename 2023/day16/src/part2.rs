use std::collections::VecDeque;

use aoclib::AocError;

use crate::part1::{amount_of_energized_tiles, parse_grid, Direction, Position};

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let grid = parse_grid(input)?;
    let first_row = (0..grid.col_num).map(|col_idx| {
        (
            Position {
                row_idx: 0,
                col_idx,
            },
            Direction::Down,
        )
    });
    let last_row = (0..grid.col_num).map(|col_idx| {
        (
            Position {
                row_idx: grid.row_num - 1,
                col_idx,
            },
            Direction::Up,
        )
    });
    let first_column = (0..grid.row_num).map(|row_idx| {
        (
            Position {
                row_idx,
                col_idx: 0,
            },
            Direction::Right,
        )
    });
    let last_column = (0..grid.row_num).map(|row_idx| {
        (
            Position {
                row_idx,
                col_idx: grid.col_num - 1,
            },
            Direction::Left,
        )
    });
    let all_outer = first_row
        .chain(first_column)
        .chain(last_row)
        .chain(last_column);
    let amounts = all_outer.map(|start| amount_of_energized_tiles(&grid, start));
    amounts
        .max()
        .ok_or_else(|| AocError::LogicError("Couldn't find a maximum".to_string()))
}
