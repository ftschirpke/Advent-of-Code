use aoclib::AocError;

use nom::character::complete::{newline, one_of};
use nom::{
    branch::alt, character::complete::char as nom_char, character::complete::i32 as nom_i32,
    multi::many1, multi::many1_count, IResult,
};

#[derive(Debug, Clone)]
pub struct SchematicPart {
    pub idx: usize,
    pub kind: PartKind,
}

impl SchematicPart {
    fn is_number(&self) -> bool {
        matches!(self.kind, PartKind::Number { .. })
    }

    fn is_symbol(&self) -> bool {
        matches!(self.kind, PartKind::Symbol { .. })
    }
}

#[derive(Debug, Clone)]
pub enum PartKind {
    Periods { width: usize },
    Symbol { symbol: char },
    Number { width: usize, value: i32 },
}

impl PartKind {
    pub fn width(&self) -> usize {
        match self {
            PartKind::Periods { width } => *width,
            PartKind::Symbol { .. } => 1,
            PartKind::Number { width, .. } => *width,
        }
    }
}

fn parse_periods(input: &str) -> IResult<&str, PartKind> {
    let (input, period_count) = many1_count(nom_char('.'))(input)?;
    Ok((
        input,
        PartKind::Periods {
            width: period_count,
        },
    ))
}

fn parse_symbol(input: &str) -> IResult<&str, PartKind> {
    let (input, symbol) = one_of("$%&/+-*#=@")(input)?;
    Ok((input, PartKind::Symbol { symbol }))
}

fn parse_number(input: &str) -> IResult<&str, PartKind> {
    let initial_length = input.len();
    let (input, number) = nom_i32(input)?;
    let number_length = initial_length - input.len();
    Ok((
        input,
        PartKind::Number {
            width: number_length,
            value: number,
        },
    ))
}

fn parse_row(input: &str) -> IResult<&str, Vec<SchematicPart>> {
    let (input, row_part_kinds) = many1(alt((parse_periods, parse_symbol, parse_number)))(input)?;
    let mut parts_width = 0;
    let parts = row_part_kinds
        .into_iter()
        .map(|kind| {
            let part = SchematicPart {
                idx: parts_width,
                kind,
            };
            parts_width += part.kind.width();
            part
        })
        .collect::<Vec<_>>();
    let (input, _) = newline(input)?;
    Ok((input, parts))
}

pub fn parse_schematic(input: &str) -> Result<Vec<Vec<SchematicPart>>, AocError> {
    let (rest, schematic) = many1(parse_row)(input)?;
    if rest.is_empty() {
        Ok(schematic)
    } else {
        Err(AocError::ParseError(format!("Parsing left over: {}", rest)))
    }
}

fn any_adjacent_symbol(focused_part: &SchematicPart, row: &[SchematicPart]) -> bool {
    row.iter().any(|part| {
        part.is_symbol() && is_adjacent(focused_part.idx, focused_part.kind.width(), part.idx)
    })
}

pub fn is_adjacent(number_idx: usize, number_width: usize, symbol_idx: usize) -> bool {
    symbol_idx + 1 >= number_idx && symbol_idx <= number_idx + number_width
}

fn valid_numbers_from_row(
    row: &Vec<SchematicPart>,
    prev_row: Option<&Vec<SchematicPart>>,
    next_row: Option<&Vec<SchematicPart>>,
) -> Result<Vec<i32>, AocError> {
    let row_length = row.len();
    let mut valid_numbers: Vec<i32> = Vec::new();
    let numbers_iter = row.iter().enumerate().filter(|(_, part)| part.is_number());
    for (idx_in_row, number_part) in numbers_iter {
        let is_valid = idx_in_row >= 1 && row[idx_in_row - 1].is_symbol()
            || idx_in_row < row_length - 1 && row[idx_in_row + 1].is_symbol()
            || prev_row.is_some() && any_adjacent_symbol(number_part, prev_row.unwrap())
            || next_row.is_some() && any_adjacent_symbol(number_part, next_row.unwrap());
        if is_valid {
            match number_part.kind {
                PartKind::Number { value, .. } => valid_numbers.push(value),
                _ => unreachable!(),
            }
        }
    }
    Ok(valid_numbers)
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let schematic = parse_schematic(input)?;
    let mut valid_numbers: Vec<i32> = Vec::new();
    for (row_idx, row) in schematic.iter().enumerate() {
        let prev_row = if row_idx > 0 {
            Some(&schematic[row_idx - 1])
        } else {
            None
        };
        let next_row = if row_idx < schematic.len() - 1 {
            Some(&schematic[row_idx + 1])
        } else {
            None
        };
        let row_valid_numbers = valid_numbers_from_row(row, prev_row, next_row)?;
        valid_numbers.extend(row_valid_numbers);
    }

    let sum: i32 = valid_numbers.iter().sum();
    Ok(sum)
}
