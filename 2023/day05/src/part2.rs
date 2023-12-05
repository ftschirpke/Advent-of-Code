use aoclib::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64 as u64_parser},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::almanac::{parse_almanac, Range};

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) =
        separated_list1(space1, separated_pair(u64_parser, space1, u64_parser))(input)?;
    let (input, _) = newline(input)?;
    let seeds = seeds
        .into_iter()
        .map(|(range_start, range_length)| Range::new(range_start, range_length))
        .collect();
    Ok((input, seeds))
}

pub fn process(input: &'static str) -> Result<u64, AocError> {
    let (input, seed_ranges) = parse_seeds(input)?;
    let almanac = parse_almanac(input)?;
    if seed_ranges.is_empty() {
        return Err(AocError::LogicError(
            "Seeds list should contain at least one seed range".to_string(),
        ));
    }
    let smallest_mapped_locations = seed_ranges.iter().map(|range| {
        let mut location_ranges = almanac.location_ranges_for_seed_range(range);
        location_ranges.sort_by_key(|range| range.start);
        location_ranges.first().unwrap().start
    });
    smallest_mapped_locations.min().ok_or(AocError::LogicError(
        "Minimum should exist for vector of length > 0".to_string(),
    ))
}
