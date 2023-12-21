use std::collections::HashSet;

use aoclib::direction::Direction;
use aoclib::grid::Grid;
use aoclib::position::Position;
use aoclib::AocError;
use nom::{branch::alt, character::complete::char as char_parser, combinator::value, IResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Start,
    Garden,
    Rocks,
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> char {
        match tile {
            Tile::Start => 'S',
            Tile::Garden => '.',
            Tile::Rocks => '#',
        }
    }
}

pub fn parse_tile(input: &str) -> IResult<&str, Tile> {
    alt((
        value(Tile::Start, char_parser('S')),
        value(Tile::Garden, char_parser('.')),
        value(Tile::Rocks, char_parser('#')),
    ))(input)
}

pub fn step_once(
    grid: &Grid<Tile>,
    current_positions: &mut HashSet<Position>,
    previous_positions: &mut HashSet<Position>,
) {
    let new_positions: HashSet<Position> = current_positions
        .iter()
        .flat_map(|pos| Direction::each().filter_map(|dir| pos.in_direction(dir)))
        .filter(|new_pos| {
            if let Some(tile) = grid.get_pos(new_pos) {
                match tile {
                    Tile::Garden | Tile::Start => !previous_positions.contains(new_pos),
                    Tile::Rocks => false,
                }
            } else {
                false
            }
        })
        .collect();
    std::mem::swap(previous_positions, current_positions);
    *current_positions = new_positions;
}

pub fn count_garden_plots_for_fixed_step_amount(
    grid: &Grid<Tile>,
    step_count: usize,
) -> Result<usize, AocError> {
    let starting_position = (0..grid.rows)
        .find_map(|row_num| {
            grid.row(row_num)
                .position(|&tile| tile == Tile::Start)
                .map(|col_num| (row_num, col_num))
        })
        .map(|(row, col)| Position::new(row, col));
    if starting_position.is_none() {
        return Err(AocError::ParseError(
            "No starting position found".to_string(),
        ));
    }
    let starting_position = starting_position.unwrap();
    let mut current_positions: HashSet<Position> = HashSet::from([starting_position]);
    let mut previous_positions: HashSet<Position> = HashSet::new();
    let step_count_mod2 = step_count % 2;
    let mut garden_plots_reached = if step_count_mod2 == 0 { 1 } else { 0 }; // starting position is a garden plot
    (1..=step_count).for_each(|step_num| {
        step_once(grid, &mut current_positions, &mut previous_positions);
        if step_num % 2 == step_count_mod2 {
            garden_plots_reached += current_positions.len();
        }
    });
    Ok(garden_plots_reached)
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let grid = Grid::parse_from(input, parse_tile)?;
    count_garden_plots_for_fixed_step_amount(&grid, 64)
}
