use std::{cmp::min, collections::VecDeque};

use aoclib::AocError;
use nom::{
    branch::alt,
    character::complete::{char as char_parser, line_ending},
    combinator::value,
    multi::many1,
    IResult,
};

use crate::grid::{Direction, Grid, Location, Pipe};

fn pipe_parser(input: &str) -> IResult<&str, Pipe> {
    alt((
        value(Pipe::StartingPosition, char_parser('S')),
        value(Pipe::LeftToRight, char_parser('-')),
        value(Pipe::TopToBottom, char_parser('|')),
        value(Pipe::LeftToTop, char_parser('J')),
        value(Pipe::LeftToBottom, char_parser('7')),
        value(Pipe::RightToTop, char_parser('L')),
        value(Pipe::RightToBottom, char_parser('F')),
        value(Pipe::None, char_parser('.')),
    ))(input)
}

fn parse_grid_row(input: &str) -> IResult<&str, Vec<Pipe>> {
    let (input, pipes) = many1(pipe_parser)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, pipes))
}

pub fn parse_grid(input: &str) -> Result<Grid, AocError> {
    let (rest, rows) = many1(parse_grid_row)(input)?;
    if rest.is_empty() {
        Grid::new(rows)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing the grid left rest: {}",
            rest
        )))
    }
}

#[derive(Debug)]
pub struct Step {
    from: Location,
    direction: Direction,
    to: Location,
}

pub fn check_step(grid: &Grid, current_location: &Location, direction: Direction) -> Option<Step> {
    let current_pipe = grid.get(current_location)?;
    if !current_pipe.connects(&direction) {
        return None;
    }
    let next_location = current_location.in_direction(direction)?;
    let next_pipe = grid.get(&next_location)?;
    if next_pipe.connects(&direction.opposite()) {
        Some(Step {
            from: *current_location,
            direction,
            to: next_location,
        })
    } else {
        None
    }
}

pub fn calculate_main_loop(grid: &Grid) -> Result<VecDeque<Location>, AocError> {
    let mut main_loop = VecDeque::new();
    let mut steps_to_check: VecDeque<Step> = Direction::each()
        .filter_map(|direction| check_step(grid, &grid.starting_location, direction))
        .collect();
    loop {
        let step = steps_to_check.pop_back().unwrap();
        if step.to == grid.starting_location {
            break;
        }
        loop {
            if let Some(last_position) = main_loop.back() {
                if *last_position == step.from {
                    break;
                }
                main_loop.pop_back();
            } else if step.from == grid.starting_location {
                break;
            } else {
                return Err(AocError::LogicError(
                    "No path found to starting location".to_string(),
                ));
            }
        }
        let mut possible_steps_from_here = Direction::each()
            .filter(|direction| *direction != step.direction.opposite()) // don't go back
            .filter_map(|direction| check_step(grid, &step.to, direction)) // can I go there?
            .filter(|step| !main_loop.contains(&step.to)) // haven't been there before?
            .peekable();
        let step_is_possible = possible_steps_from_here.peek().is_some();
        if step_is_possible {
            steps_to_check.extend(possible_steps_from_here);
            main_loop.push_back(step.to);
        } else if steps_to_check.is_empty() {
            return Err(AocError::LogicError(
                "No path found to starting location (no steps left to check)".to_string(),
            ));
        }
    }
    Ok(main_loop)
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let grid = parse_grid(input)?;
    let main_loop = calculate_main_loop(&grid)?;
    Ok((main_loop.len() + 1) / 2)
}
