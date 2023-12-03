use std::str::FromStr;

use aoclib::AocError;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i32 as i32number, combinator::value,
    multi::separated_list1, sequence::delimited, sequence::separated_pair, IResult,
};

#[derive(Debug)]
pub struct Game {
    id: i32,
    pub sets: Vec<Set>,
}

#[derive(Debug)]
pub struct Set {
    pub red: i32,
    pub blue: i32,
    pub green: i32,
}

impl Set {
    pub fn new() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

impl Default for Set {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
enum Color {
    Red,
    Blue,
    Green,
}

fn game_parser(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), i32number, tag(": "))(input)?;
    let (input, sets) = separated_list1(tag("; "), set_parser)(input)?;
    let game = Game { id, sets };
    Ok((input, game))
}

fn set_parser(input: &str) -> IResult<&str, Set> {
    let (input, color_pairs) = separated_list1(tag(", "), color_pair_parser)(input)?;
    let mut set = Set::new();
    for (val, color) in color_pairs.iter() {
        match color {
            Color::Red => set.red += val,
            Color::Blue => set.blue += val,
            Color::Green => set.green += val,
        }
    }
    Ok((input, set))
}

fn color_pair_parser(input: &str) -> IResult<&str, (i32, Color)> {
    separated_pair(i32number, tag(" "), color_parser)(input)
}

fn color_parser(input: &str) -> IResult<&str, Color> {
    let red = value(Color::Red, tag("red"));
    let blue = value(Color::Blue, tag("blue"));
    let green = value(Color::Green, tag("green"));
    alt((red, blue, green))(input)
}

impl FromStr for Game {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, game) = game_parser(s)?;
        if rest.is_empty() {
            Ok(game)
        } else {
            Err(AocError::ParseError(format!(
                "Game parser did not consume all input: {}",
                rest
            )))
        }
    }
}

pub fn game_id_if_possible(line: &str) -> Result<Option<i32>, AocError> {
    let game = line.parse::<Game>()?;
    let possible = game
        .sets
        .iter()
        .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14);
    if possible {
        Ok(Some(game.id))
    } else {
        Ok(None)
    }
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    input.lines().try_fold(0i32, |count, line| {
        if let Some(id) = game_id_if_possible(line)? {
            Ok(count + id)
        } else {
            Ok(count)
        }
    })
}
