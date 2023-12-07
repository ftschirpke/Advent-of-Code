use std::collections::HashMap;

use aoclib::AocError;
use nom::character::complete::space1;
use nom::multi::fill;
use nom::IResult;
use nom::{
    character::complete::{newline, one_of, u32 as u32_parser},
    multi::separated_list1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScoreType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub trait Scoreable {
    fn score(&self) -> ScoreType;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    kinds: [Kind; 5],
}

impl Scoreable for Hand {
    fn score(&self) -> ScoreType {
        let mut counts: HashMap<Kind, u32> = HashMap::new();
        for kind in self.kinds.iter() {
            let entry = counts.entry(*kind).or_insert(0);
            *entry += 1;
        }
        match counts.len() {
            1 => ScoreType::FiveOfAKind,
            2 => {
                if counts.values().any(|&v| v == 4) {
                    ScoreType::FourOfAKind
                } else {
                    ScoreType::FullHouse
                }
            }
            3 => {
                if counts.values().any(|&v| v == 3) {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandInfo<H: Scoreable> {
    pub score_type: ScoreType,
    hand: H,
    pub bid: u32,
}

impl<H: Scoreable> HandInfo<H> {
    pub fn new(hand: H, bid: u32) -> Self {
        Self {
            score_type: hand.score(),
            hand,
            bid,
        }
    }
}

fn parse_kind(input: &str) -> IResult<&str, Kind> {
    let (input, c) = one_of("23456789TJQKA")(input)?;
    let kind = match c {
        'A' => Kind::Ace,
        'K' => Kind::King,
        'Q' => Kind::Queen,
        'J' => Kind::Jack,
        'T' => Kind::Ten,
        '9' => Kind::Nine,
        '8' => Kind::Eight,
        '7' => Kind::Seven,
        '6' => Kind::Six,
        '5' => Kind::Five,
        '4' => Kind::Four,
        '3' => Kind::Three,
        '2' => Kind::Two,
        _ => unreachable!(),
    };
    Ok((input, kind))
}

pub fn parse_hand_and_bid(input: &str) -> IResult<&str, (Hand, u32)> {
    let mut buf = [Kind::Two; 5];
    let (input, ()) = fill(parse_kind, &mut buf)(input)?;
    let hand = Hand { kinds: buf };
    let (input, _) = space1(input)?;
    let (input, bid) = u32_parser(input)?;
    Ok((input, (hand, bid)))
}

pub fn parse_hand_infos(input: &str) -> Result<Vec<HandInfo<Hand>>, AocError> {
    let (input, hands_and_bids) = separated_list1(newline, parse_hand_and_bid)(input)?;
    let (rest, _) = newline(input)?;
    if rest.is_empty() {
        let hand_infos = hands_and_bids
            .into_iter()
            .map(|(hand, bid)| HandInfo {
                score_type: hand.score(),
                hand,
                bid,
            })
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
