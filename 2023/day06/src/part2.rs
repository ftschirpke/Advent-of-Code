use aoclib::AocError;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::preceded,
};

use crate::part1::{best_time_held, Record};

fn parse_record(input: &str) -> Result<Record, AocError> {
    let (input, _) = preceded(tag("Time:"), space1)(input)?;
    let (input, time_parts) = separated_list1(space1, digit1)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = preceded(tag("Distance:"), space1)(input)?;
    let (input, distance_parts) = separated_list1(space1, digit1)(input)?;
    let (rest, _) = newline(input)?;
    let time_str = time_parts.into_iter().collect::<String>();
    dbg!(&time_str);
    let distance_str = distance_parts.into_iter().collect::<String>();
    dbg!(&distance_str);
    let time = time_str
        .parse::<u64>()
        .map_err(|_| AocError::ParseError(format!("Failed to parse time: {}", time_str)))?;
    let distance = distance_str
        .parse::<u64>()
        .map_err(|_| AocError::ParseError(format!("Failed to parse distance: {}", distance_str)))?;
    if rest.is_empty() {
        Ok(Record { time, distance })
    } else {
        Err(AocError::ParseError(format!(
            "Parsing of time and distance left rest: {}",
            rest
        )))
    }
}

// Let's calculate the time_held for a given distance
// (race_time - time_held) * time_held == distance
// -> time_held^2 - race_time * time_held + distance == 0
// -> time_held = race_time / 2 +- sqrt( race_time^2 / 4 - distance )

pub fn process(input: &'static str) -> Result<u64, AocError> {
    let record = parse_record(input)?;
    let (best_time_held, alternative) = best_time_held(record.time);
    let number_of_maxima = if alternative.is_some() { 2 } else { 1 };
    let helper = (record.time as f64).powi(2) / 4.0 - record.distance as f64;
    let helper = helper.sqrt() as u64;
    if helper <= 1 {
        return Ok(number_of_maxima);
    }
    let number_of_better_solutions = number_of_maxima + helper * 2;
    Ok(number_of_better_solutions)
}
