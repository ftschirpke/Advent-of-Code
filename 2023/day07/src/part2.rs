use std::collections::HashMap;

use aoclib::AocError;
use nom::character::complete::space1;
use nom::multi::fill;
use nom::IResult;
use nom::{
    character::complete::{newline, one_of, u32 as u32_parser},
    multi::separated_list1,
};

use crate::part1::{HandInfo, ScoreType, Scoreable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JokerKind {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JokerHand {
    kinds: [JokerKind; 5],
}

impl Scoreable for JokerHand {
    fn score(&self) -> ScoreType {
        let mut counts: HashMap<JokerKind, u32> = HashMap::new();
        for kind in self.kinds.iter() {
            let entry = counts.entry(*kind).or_insert(0);
            *entry += 1;
        }
        let joker_count = counts.remove(&JokerKind::Joker).unwrap_or(0);
        match counts.len() {
            0 | 1 => ScoreType::FiveOfAKind,
            2 => {
                if counts.values().any(|&v| v == 4 - joker_count) {
                    ScoreType::FourOfAKind
                } else {
                    ScoreType::FullHouse
                }
            }
            3 => {
                if counts.values().any(|&v| v == 3 - joker_count) {
                    ScoreType::ThreeOfAKind
                } else {
                    ScoreType::TwoPairs
                }
            }
            4 => ScoreType::OnePair,
            5 => ScoreType::HighCard,
            _ => unreachable!(),
        }
    }
}

fn parse_joker_kind(input: &str) -> IResult<&str, JokerKind> {
    let (input, c) = one_of("23456789TJQKA")(input)?;
    let kind = match c {
        'A' => JokerKind::Ace,
        'K' => JokerKind::King,
        'Q' => JokerKind::Queen,
        'T' => JokerKind::Ten,
        '9' => JokerKind::Nine,
        '8' => JokerKind::Eight,
        '7' => JokerKind::Seven,
        '6' => JokerKind::Six,
        '5' => JokerKind::Five,
        '4' => JokerKind::Four,
        '3' => JokerKind::Three,
        '2' => JokerKind::Two,
        'J' => JokerKind::Joker,
        _ => unreachable!(),
    };
    Ok((input, kind))
}

pub fn parse_joker_hand_and_bid(input: &str) -> IResult<&str, (JokerHand, u32)> {
    let mut buf = [JokerKind::Two; 5];
    let (input, ()) = fill(parse_joker_kind, &mut buf)(input)?;
    let hand = JokerHand { kinds: buf };
    let (input, _) = space1(input)?;
    let (input, bid) = u32_parser(input)?;
    Ok((input, (hand, bid)))
}

pub fn parse_hand_infos(input: &str) -> Result<Vec<HandInfo<JokerHand>>, AocError> {
    let (input, hands_and_bids) = separated_list1(newline, parse_joker_hand_and_bid)(input)?;
    let (rest, _) = newline(input)?;
    if rest.is_empty() {
        let hand_infos = hands_and_bids
            .into_iter()
            .map(|(hand, bid)| HandInfo::new(hand, bid))
            .collect::<Vec<_>>();
        Ok(hand_infos)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing hand infos left rest: {}",
            rest
        )))
    }
}

pub fn process(input: &'static str) -> Result<u32, AocError> {
    let mut hand_infos = parse_hand_infos(input)?;
    hand_infos.sort();
    let accum = hand_infos
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, hand_info)| {
            let rank = (i + 1) as u32;
            acc + hand_info.bid * rank
        });
    Ok(accum)
}



