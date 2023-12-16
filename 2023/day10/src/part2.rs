use std::collections::VecDeque;

use aoclib::AocError;

use crate::grid::{Grid, Location, Pipe};
use crate::part1::{calculate_main_loop, parse_grid};

fn starting_position_pipe(grid: &Grid, main_loop: &VecDeque<Location>) -> Option<Pipe> {
    let above = if let Some(pos) = grid.starting_location.above() {
        main_loop.contains(&pos) && grid.get(&pos).unwrap().connects_bottom()
    } else {
        false
    };
    let below = if let Some(pos) = grid.starting_location.below() {
        main_loop.contains(&pos) && grid.get(&pos).unwrap().connects_top()
    } else {
        false
    };
    let left = if let Some(pos) = grid.starting_location.left() {
        main_loop.contains(&pos) && grid.get(&pos).unwrap().connects_right()
    } else {
        false
    };
    let right = if let Some(pos) = grid.starting_location.right() {
        main_loop.contains(&pos) && grid.get(&pos).unwrap().connects_left()
    } else {
        false
    };
    match (above, below, left, right) {
        (true, true, false, false) => Some(Pipe::TopToBottom),
        (true, false, true, false) => Some(Pipe::LeftToTop),
        (true, false, false, true) => Some(Pipe::RightToTop),
        (false, true, true, false) => Some(Pipe::LeftToBottom),
        (false, true, false, true) => Some(Pipe::RightToBottom),
        (false, false, true, true) => Some(Pipe::LeftToRight),
        _ => None,
    }
}

fn handle_border_pipes(inside: &mut bool, pipe: Pipe, border_start: &mut Option<Pipe>) {
    if let Some(first_border_pipe) = border_start {
        match pipe {
            Pipe::LeftToRight => {
                // still riding along the border
            },
            Pipe::LeftToTop => match *first_border_pipe {            
                Pipe::RightToTop => {
                    // rode along the border of the main-loop but did not cross it
                    *border_start = None;
                }
                Pipe::RightToBottom => {
                    // crossed a border of the main-loop
                    *inside = !*inside;
                    *border_start = None;
                }
                _ => unreachable!("currently riding along a border from left to right, border start pipe must be RightToTop or RightToBottom"),
            },
            Pipe::LeftToBottom  => match *first_border_pipe {            
                Pipe::RightToTop => {
                    // crossed a border of the main-loop
                    *inside = !*inside;
                    *border_start = None;
                }
                Pipe::RightToBottom => {
                    // rode along the border of the main-loop but did not cross it
                    *border_start = None;
                }
                _ => unreachable!("currently riding along a border from left to right, border start pipe must be RightToTop or RightToBottom"),
            }
            _ => unreachable!("currently riding along a border from left to right, every pipe should be open to left"),
        }
     } else {
         match pipe {
             Pipe::RightToBottom | Pipe::RightToTop => { 
                 *border_start = Some(pipe);
             },
             Pipe::TopToBottom => {
                 *inside = !*inside;
             },
             _ => unreachable!("currently not riding along a border, main border pipe must connect to top and/or bottom"),
         }
     }
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let grid = parse_grid(input)?;
    let main_loop = calculate_main_loop(&grid)?;
    let start_pos_pipe = starting_position_pipe(&grid, &main_loop).ok_or_else(|| {
        AocError::LogicError("Could not determine pipe for starting position".to_string())
    })?;
    let main_loop = {
        let mut empty_grid = vec![vec![false; grid.cols]; grid.rows];
        empty_grid[grid.starting_location.row][grid.starting_location.col] = true;
        main_loop.into_iter().for_each(|pos| empty_grid[pos.row][pos.col] = true);
        empty_grid
    };
    let inside_count: usize = (0..grid.rows)
        .map(|row_num| {
            let mut border_start: Option<Pipe> = None;
            let mut inside = false;
            (0..grid.cols)
                .filter(|col_num| {
                    let pipe = if row_num == grid.starting_location.row
                        && *col_num == grid.starting_location.col
                    {
                        start_pos_pipe
                    } else {
                        grid.get_by_idx(row_num, *col_num).unwrap()
                    };
                    if main_loop[row_num][*col_num] {
                        handle_border_pipes(&mut inside, pipe, &mut border_start);
                        false // this is a pipe in the main loop, thus not inside the main loop
                    } else { 
                        inside
                    }
                })
                .count()
        })
        .sum();

    Ok(inside_count)
}
