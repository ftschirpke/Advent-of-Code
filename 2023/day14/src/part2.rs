use std::collections::VecDeque;
use std::mem::swap;

use aoclib::AocError;

use crate::part1::{parse_grid, Grid, Rock};

fn tilt<'a>(iter: impl Iterator<Item = &'a mut Rock>) {
    let mut swappable_empty_rocks: VecDeque<&mut Rock> = VecDeque::new();
    for rock in iter {
        match *rock {
            Rock::Empty => swappable_empty_rocks.push_back(rock),
            Rock::Stationary => swappable_empty_rocks.clear(),
            Rock::Rolling => {
                let swap_partner = swappable_empty_rocks.pop_front();
                if let Some(other_rock) = swap_partner {
                    swap(rock, other_rock);
                    swappable_empty_rocks.push_back(rock);
                }
            }
        }
    }
}

fn cycle(grid: &mut Grid) {
    // tilt north
    (0..grid.cols).for_each(|col_num| tilt(grid.column_mut(col_num)));
    // tilt west
    (0..grid.rows).for_each(|row_num| tilt(grid.row_mut(row_num)));
    // tilt south
    (0..grid.cols).for_each(|col_num| tilt(grid.column_mut(col_num).rev()));
    // tilt east
    (0..grid.rows)
        .rev()
        .for_each(|row_num| tilt(grid.row_mut(row_num).rev()));
}

const ONE_BILLION: usize = 1_000_000_000;

fn load(grid: &Grid) -> i32 {
    (0..grid.cols)
        .map(|col_num| {
            let column_pressure: i32 = grid
                .column(col_num)
                .enumerate()
                .filter_map(|(i, rock)| {
                    if *rock == Rock::Rolling {
                        Some((grid.rows - i) as i32)
                    } else {
                        None
                    }
                })
                .sum();
            column_pressure
        })
        .sum()
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let mut grid = parse_grid(input)?;
    let mut seen_grids = Vec::from([grid.clone()]);
    let first = loop {
        cycle(&mut grid);
        let first_occurence = seen_grids.iter().position(|seen_grid| *seen_grid == grid);
        if let Some(first) = first_occurence {
            break first;
        } else {
            seen_grids.push(grid.clone());
        }
    };
    let repetition_after = seen_grids.len() - first;
    let cycles_after_last_repetition = (ONE_BILLION - first) % repetition_after;
    let final_grid = &seen_grids[first + cycles_after_last_repetition];
    Ok(load(final_grid))
}
