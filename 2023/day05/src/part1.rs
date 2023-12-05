use aoclib::AocError;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64 as u64_parser},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

use crate::almanac::parse_almanac;

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("seeds: "), separated_list1(space1, u64_parser), newline)(input)
}

pub fn process(input: &'static str) -> Result<u64, AocError> {
    let (input, seeds) = parse_seeds(input)?;
    let almanac = parse_almanac(input)?;
    let seed_locations = seeds
        .into_iter()
        .map(|seed| almanac.location_for_seed(seed));
    match seed_locations.len() {
        0 => Err(AocError::LogicError(
            "There should be one location for each seed but found 0 locations".to_string(),
        )),
        _ => seed_locations.min().ok_or(AocError::LogicError(
            "Minimum should exist for vector of length > 0".to_string(),
        )),
    }
}
