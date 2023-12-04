use aoclib::AocError;

use nom::{
    bytes::complete::tag,
    character::complete::i32 as i32_parser,
    multi::{many1_count, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

pub struct Card {
    pub id: i32,
    winning_numbers: Vec<i32>,
    your_numbers: Vec<i32>,
}

fn spaces(input: &str) -> IResult<&str, usize> {
    many1_count(tag(" "))(input)
}

fn number_list_parser(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(spaces, i32_parser)(input)
}

fn separator(input: &str) -> IResult<&str, &str> {
    delimited(spaces, tag("|"), spaces)(input)
}

fn card_parser(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = spaces(input)?;
    let (input, id) = i32_parser(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = spaces(input)?;
    let (input, (winning_numbers, your_numbers)) =
        separated_pair(number_list_parser, separator, number_list_parser)(input)?;
    Ok((
        input,
        Card {
            id,
            winning_numbers,
            your_numbers,
        },
    ))
}

pub fn parse_card(line: &str) -> Result<Card, AocError> {
    let (rest, card) = card_parser(line)?;
    if rest.is_empty() {
        Ok(card)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing left over rest: {}",
            rest
        )))
    }
}

pub fn number_of_matches(card: &Card) -> i32 {
    card.your_numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count() as i32
}

pub fn card_value(card: &Card) -> i32 {
    let matches = number_of_matches(card) as u32;
    if matches == 0 {
        0
    } else {
        2i32.pow(matches - 1)
    }
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    input
        .lines()
        .map(parse_card)
        .try_fold(0i32, |acc, card| Ok(acc + card_value(&card?)))
}
