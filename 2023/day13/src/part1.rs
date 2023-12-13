use aoclib::AocError;

use bitvec::{bitvec, vec::BitVec};
use nom::{
    branch::alt,
    character::complete::{char as char_parser, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug)]
pub struct Terrain {
    pub rows: Vec<BitVec>,
    pub cols: Vec<BitVec>,
}

impl Terrain {
    pub fn new(bool_rows: Vec<Vec<bool>>) -> Self {
        let mut cols = vec![bitvec![0; bool_rows.len()]; bool_rows[0].len()];
        bool_rows.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &el)| el)
                .for_each(|(col_idx, _)| {
                    cols[col_idx].set(row_idx, true);
                })
        });
        let rows = bool_rows
            .into_iter()
            .map(|row| row.iter().collect::<BitVec>())
            .collect();
        Self { rows, cols }
    }
}

fn terrain_element_parser(input: &str) -> IResult<&str, bool> {
    alt((
        value(true, char_parser('#')),
        value(false, char_parser('.')),
    ))(input)
}

fn terrain_row_parser(input: &str) -> IResult<&str, Vec<bool>> {
    let (input, row) = many1(terrain_element_parser)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, row))
}

fn terrain_parser(input: &str) -> IResult<&str, Terrain> {
    let (input, boolean_rows) = many1(terrain_row_parser)(input)?;
    Ok((input, Terrain::new(boolean_rows)))
}

pub fn parse_all(input: &str) -> Result<Vec<Terrain>, AocError> {
    let (rest, terrains) = separated_list1(line_ending, terrain_parser)(input)?;
    if rest.is_empty() {
        Ok(terrains)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing all the terrains left rest: {}",
            rest
        )))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorAxis {
    Horizontal(usize),
    Vertical(usize),
}

pub fn find_mirror_index(bitvecs: &[BitVec]) -> Option<usize> {
    let len = bitvecs.len();
    (0..len - 1).find(|idx| {
        let mut before_idx = *idx;
        let mut after_idx = *idx + 1;
        while bitvecs[before_idx] == bitvecs[after_idx] {
            if before_idx == 0 || after_idx == len - 1 {
                return true;
            }
            before_idx -= 1;
            after_idx += 1;
        }
        false
    })
}

pub fn find_mirror_axis(terrain: &Terrain) -> Option<MirrorAxis> {
    find_mirror_index(&terrain.rows)
        .map(MirrorAxis::Horizontal)
        .or_else(|| find_mirror_index(&terrain.cols).map(MirrorAxis::Vertical))
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let terrains = parse_all(input)?;
    let score = terrains
        .iter()
        .filter_map(find_mirror_axis)
        .fold(0usize, |acc, axis| {
            acc + match axis {
                MirrorAxis::Horizontal(idx) => 100 * (idx + 1),
                MirrorAxis::Vertical(idx) => idx + 1,
            }
        });
    Ok(score)
}
