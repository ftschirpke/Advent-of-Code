use aoclib::AocError;
use nom::{
    branch::alt,
    character::complete::{alpha1, char as char_parser, line_ending, u8 as u8_parser},
    combinator::{opt, value},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Copy, Clone)]
pub enum EntryEnding {
    Minus,
    Equal(u8),
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub character_sequence: &'static str,
    pub ending: EntryEnding,
}

fn parse_minus(input: &str) -> IResult<&str, EntryEnding> {
    value(EntryEnding::Minus, char_parser('-'))(input)
}

fn parse_equal(input: &str) -> IResult<&str, EntryEnding> {
    let (input, number) = preceded(char_parser('='), u8_parser)(input)?;
    Ok((input, EntryEnding::Equal(number)))
}
fn parse_entry(input: &'static str) -> IResult<&str, Entry> {
    let (input, character_sequence) = alpha1(input)?;
    let (input, ending) = alt((parse_minus, parse_equal))(input)?;
    let entry = Entry {
        character_sequence,
        ending,
    };
    Ok((input, entry))
}

pub fn parse_input(input: &'static str) -> Result<Vec<Entry>, AocError> {
    let (rest, entries) = separated_list1(char_parser(','), parse_entry)(input)?;
    let (rest, _) = opt(line_ending)(rest)?;
    if rest.is_empty() {
        Ok(entries)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing the entries left rest: {}",
            rest
        )))
    }
}

pub fn ascii(c: char) -> u32 {
    c as u32
}

pub fn char_hash(acc: u32, c: char) -> u32 {
    let val = acc + ascii(c);
    let val = val * 17;
    val % 256
}

pub fn hash(entry: &Entry) -> u32 {
    let character_hash = entry.character_sequence.chars().fold(0u32, char_hash);
    match entry.ending {
        EntryEnding::Minus => char_hash(character_hash, '-'),
        EntryEnding::Equal(num) => {
            let hash_after_equal_sign = char_hash(character_hash, '=');
            num.to_string()
                .drain(..)
                .fold(hash_after_equal_sign, char_hash)
        }
    }
}

pub fn process(input: &'static str) -> Result<u32, AocError> {
    let entries = parse_input(input)?;
    let hash_result: u32 = entries.iter().map(hash).sum();
    Ok(hash_result)
}
