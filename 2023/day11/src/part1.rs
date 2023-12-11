use std::collections::{BTreeMap, BTreeSet};

use aoclib::AocError;
use nom::{
    branch::alt,
    character::complete::{char as char_parser, line_ending},
    multi::{many1, many1_count},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
enum ParseResult {
    Galaxy,
    EmptySpace(usize),
}

fn empty_space_parser(input: &str) -> IResult<&str, ParseResult> {
    let (input, count) = many1_count(char_parser('.'))(input)?;
    Ok((input, ParseResult::EmptySpace(count)))
}

fn galaxy_parser(input: &str) -> IResult<&str, ParseResult> {
    let (input, _) = char_parser('#')(input)?;
    Ok((input, ParseResult::Galaxy))
}

fn row_parser(input: &str) -> IResult<&str, Vec<(usize, ParseResult)>> {
    let (input, row_results) = many1(alt((empty_space_parser, galaxy_parser)))(input)?;
    let (input, _) = line_ending(input)?;
    let mut idx: usize = 0;
    let row: Vec<_> = row_results
        .into_iter()
        .map(|result| {
            let result_with_index = (idx, result);
            match result {
                ParseResult::EmptySpace(count) => idx += count,
                ParseResult::Galaxy => idx += 1,
            }
            result_with_index
        })
        .collect();
    Ok((input, row))
}

pub fn adjust_galaxy_positions(galaxy_positions: Vec<Vec<(usize, usize)>>) -> Vec<Position> {
    let mut row_num = 0;
    let mut occupied_columns = BTreeSet::new();
    let galaxy_positions: Vec<_> = galaxy_positions
        .into_iter()
        .flat_map(|row| {
            let row_num_increase = if row.is_empty() { 2 } else { 1 };
            let positions: Vec<_> = row
                .into_iter()
                .map(|(_, col)| {
                    occupied_columns.insert(col);
                    (row_num, col)
                })
                .collect();
            row_num += row_num_increase;
            positions
        })
        .collect();
    let max_column = *occupied_columns.iter().max().unwrap() as i32;
    let mut real_column = 0;
    let column_mappings: BTreeMap<_, _> = (0..=max_column)
        .filter_map(|col| {
            let col = col as usize;
            if occupied_columns.contains(&col) {
                let mapped_column = real_column;
                real_column += 1;
                Some((col, mapped_column))
            } else {
                real_column += 2;
                None
            }
        })
        .collect();
    galaxy_positions
        .into_iter()
        .map(|(row, col)| Position {
            row,
            col: column_mappings[&col],
        })
        .collect()
}

pub fn parse_galaxy_positions(input: &'static str) -> Result<Vec<Vec<(usize, usize)>>, AocError> {
    let (rest, parsed_results) = many1(row_parser)(input)?;
    if !rest.is_empty() {
        return Err(AocError::ParseError(format!(
            "Parsing galaxies left rest: '{}'",
            rest
        )));
    }
    let galaxy_positions: Vec<Vec<(usize, usize)>> = parsed_results
        .into_iter()
        .map(|row_results| {
            row_results
                .into_iter()
                .enumerate()
                .filter_map(|(row, (col, result))| match result {
                    ParseResult::Galaxy => Some((row, col)),
                    ParseResult::EmptySpace(_) => None,
                })
                .collect()
        })
        .collect();
    Ok(galaxy_positions)
}

pub fn galaxy_distance(galaxy1_pos: &Position, galaxy2_pos: &Position) -> i32 {
    (galaxy1_pos.row as i32 - galaxy2_pos.row as i32).abs()
        + (galaxy1_pos.col as i32 - galaxy2_pos.col as i32).abs()
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let galaxy_positions = parse_galaxy_positions(input)?;
    let galaxy_positions = adjust_galaxy_positions(galaxy_positions);
    let res = galaxy_positions
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy)| {
            galaxy_positions
                .iter()
                .skip(i + 1)
                .filter(move |other_galaxy| *other_galaxy != galaxy)
                .map(move |other_galaxy| galaxy_distance(galaxy, other_galaxy))
        })
        .sum();
    Ok(res)
}
