use std::num::ParseIntError;

use aoclib::{direction::Direction, AocError};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char as char_parser, line_ending, space1, u8 as u8_parser},
    combinator::{map_res, value},
    multi::many1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use crate::part1::{
    area, direction_parser as old_direction_parser, is_hex_digit, Color, DigInstruction,
};

fn count_parser(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(5, 5, is_hex_digit), from_hex)(input)
}

fn direction_parser(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Right, char_parser('0')),
        value(Direction::Left, char_parser('2')),
        value(Direction::Up, char_parser('3')),
        value(Direction::Down, char_parser('1')),
    ))(input)
}

fn from_hex(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 16)
}

fn hex_parser(input: &str) -> IResult<&str, (u32, Direction)> {
    delimited(
        tag("(#"),
        tuple((count_parser, direction_parser)),
        char_parser(')'),
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, DigInstruction> {
    let (input, (_, _)) = separated_pair(old_direction_parser, space1, u8_parser)(input)?;
    let (input, (count, direction)) = delimited(space1, hex_parser, line_ending)(input)?;
    let color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };
    let instruction = DigInstruction {
        direction,
        count,
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

pub fn process(input: &'static str) -> Result<i64, AocError> {
    let dig_instructions = parse_instructions(input)?;
    area(&dig_instructions)
}
