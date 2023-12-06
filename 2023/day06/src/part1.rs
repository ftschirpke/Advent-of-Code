use aoclib::AocError;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64 as u64_parser},
    multi::separated_list1,
    sequence::preceded,
};

pub struct Record {
    pub time: u64,
    pub distance: u64,
}

pub fn score_for(race_time: u64, time_held: u64) -> u64 {
    if time_held <= race_time {
        (race_time - time_held) * time_held
    } else {
        0
    }
}

pub fn score_derivative(race_time: u64, time_held: u64) -> u64 {
    if time_held <= race_time {
        race_time - 2 * time_held
    } else {
        0
    }
}

pub fn best_time_held(race_time: u64) -> (u64, Option<u64>) {
    let half = race_time / 2;
    if race_time % 2 == 0 {
        (half, None)
    } else {
        (half, Some(half + 1))
    }
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, AocError> {
    let (input, _) = preceded(tag("Time:"), space1)(input)?;
    let (input, times) = separated_list1(space1, u64_parser)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = preceded(tag("Distance:"), space1)(input)?;
    let (input, distances) = separated_list1(space1, u64_parser)(input)?;
    let (rest, _) = newline(input)?;
    if rest.is_empty() {
        if times.len() != distances.len() {
            return Err(AocError::ParseError(
                "Times and distances must be the same length".to_string(),
            ));
        }
        Ok(times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Record { time, distance })
            .collect())
    } else {
        Err(AocError::ParseError(format!(
            "Parsing of times and distances left rest: {}",
            rest
        )))
    }
}

pub fn process(input: &'static str) -> Result<u64, AocError> {
    let records = parse_records(input)?;
    let product = records.iter().fold(1u64, |acc, record| {
        let (max_time_held, alternative) = best_time_held(record.time);
        let mut possible_improvements: u64 = if alternative.is_some() { 2 } else { 1 };
        let mut time_held = max_time_held - 1;
        while score_for(record.time, time_held) > record.distance {
            possible_improvements += 2;
            time_held -= 1;
        }
        possible_improvements * acc
    });
    Ok(product)
}
