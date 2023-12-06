use aoclib::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u32 as u32_parser},
    multi::separated_list1,
    sequence::preceded,
};

pub struct Record {
    pub time: u32,
    pub distance: u32,
}

pub fn parse_records(input: &str) -> Result<Vec<Record>, AocError> {
    let (input, _) = preceded(tag("Time:"), space1)(input)?;
    let (input, times) = separated_list1(space1, u32_parser)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = preceded(tag("Distance:"), space1)(input)?;
    let (input, distances) = separated_list1(space1, u32_parser)(input)?;
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

pub fn process(input: &'static str) -> Result<i32, AocError> {
    todo!("Part 1");
}
