use std::num::ParseIntError;

use aoclib::direction::Direction;
use aoclib::AocError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char as char_parser, line_ending, space1, u8 as u8_parser},
    combinator::{map_res, value},
    multi::many1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigInstruction {
    pub direction: Direction,
    pub count: u32,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 16)
}

pub fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

pub fn direction_parser(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Right, char_parser('R')),
        value(Direction::Left, char_parser('L')),
        value(Direction::Up, char_parser('U')),
        value(Direction::Down, char_parser('D')),
    ))(input)
}

fn hex_val(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn color_parser(input: &str) -> IResult<&str, Color> {
    let (input, (red, green, blue)) = delimited(
        tag("(#"),
        tuple((hex_val, hex_val, hex_val)),
        char_parser(')'),
    )(input)?;
    let color = Color { red, green, blue };
    Ok((input, color))
}

fn parse_instruction(input: &str) -> IResult<&str, DigInstruction> {
    let (input, (direction, count)) = separated_pair(direction_parser, space1, u8_parser)(input)?;
    let (input, color) = delimited(space1, color_parser, line_ending)(input)?;
    let instruction = DigInstruction {
        direction,
        count: count as u32,
        color,
    };
    Ok((input, instruction))
}

fn parse_instructions(input: &'static str) -> Result<Vec<DigInstruction>, AocError> {
    let (rest, instructions) = many1(parse_instruction)(input)?;
    if rest.is_empty() {
        Ok(instructions)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing instructions left rest: {}",
            rest
        )))
    }
}

/// returns the path the dig instructions lead you to given by the vertices of the path dug
pub fn dig_path_vertices(dig_instructions: &[DigInstruction]) -> Result<Vec<(i64, i64)>, AocError> {
    let mut vertices = Vec::with_capacity(dig_instructions.len() + 1);
    let (mut last_row, mut last_col) = (0i64, 0i64);
    vertices.push((last_row, last_col));
    dig_instructions.iter().for_each(|instruction| {
        match instruction.direction {
            Direction::Left => last_col -= instruction.count as i64,
            Direction::Right => last_col += instruction.count as i64,
            Direction::Up => last_row -= instruction.count as i64,
            Direction::Down => last_row += instruction.count as i64,
        };
        vertices.push((last_row, last_col));
    });
    Ok(vertices)
}

pub fn shoelace_area(vertices: &[(i64, i64)]) -> i64 {
    vertices
        .windows(3)
        .map(|slice| {
            let (_, prev_col) = &slice[0];
            let (curr_row, _) = &slice[1];
            let (_, next_col) = &slice[2];
            curr_row * (prev_col - next_col)
        })
        .sum::<i64>()
        .abs()
        / 2
}

pub fn area(dig_instructions: &[DigInstruction]) -> Result<i64, AocError> {
    let dig_path = dig_path_vertices(dig_instructions)?;
    let path_len: i64 = dig_instructions
        .iter()
        .map(|instruction| instruction.count as i64)
        .sum();
    let shoelace_formula_area = shoelace_area(&dig_path);
    let picks_theorem_interior = shoelace_formula_area - path_len / 2 + 1;
    Ok(picks_theorem_interior + path_len)
}

pub fn process(input: &'static str) -> Result<i64, AocError> {
    let dig_instructions = parse_instructions(input)?;
    area(&dig_instructions)
}
