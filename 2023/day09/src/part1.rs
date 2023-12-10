use std::collections::VecDeque;

use aoclib::AocError;
use nom::{
    character::complete::{i32 as i32_parser, space1},
    multi::separated_list1,
};

pub fn parse_sequence(input: &str) -> Result<Vec<i32>, AocError> {
    let (rest, sequence) = separated_list1(space1, i32_parser)(input)?;
    if rest.is_empty() {
        Ok(sequence)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing sequence left over rest: {}",
            input
        )))
    }
}

pub fn differences_in_sequences(sequence: &[i32]) -> Result<Vec<i32>, AocError> {
    if sequence.len() < 2 {
        Err(AocError::LogicError(format!(
            "Cannot calculate differences of sequence with length {}",
            sequence.len()
        )))
    } else {
        let differences = sequence
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();
        Ok(differences)
    }
}

enum SequenceInfo {
    NextItem(i32),
    LastItem(i32),
}

fn try_predict_next(sequence: &[i32]) -> Result<SequenceInfo, AocError> {
    let mut iter = sequence.iter();
    let last = iter.next_back().ok_or(AocError::LogicError(
        "Cannot predict next in empty sequence".to_string(),
    ))?;
    if iter.all(|&x| x == *last) {
        Ok(SequenceInfo::NextItem(*last))
    } else {
        Ok(SequenceInfo::LastItem(*last))
    }
}

pub fn predict_next(sequence: &[i32]) -> Result<i32, AocError> {
    let mut last_elements_stack = {
        let last_element = sequence.last().ok_or(AocError::LogicError(
            "Cannot predict next in empty sequence".to_string(),
        ))?;
        VecDeque::from([*last_element])
    };
    let mut differences = differences_in_sequences(sequence)?;
    loop {
        match try_predict_next(&differences)? {
            SequenceInfo::NextItem(prediction) => {
                let prediction_for_original = last_elements_stack
                    .into_iter()
                    .rev()
                    .fold(prediction, |acc, last| acc + last);
                return Ok(prediction_for_original);
            }
            SequenceInfo::LastItem(last_item) => {
                last_elements_stack.push_back(last_item);
                differences = differences_in_sequences(&differences)?;
            }
        }
    }
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    input
        .lines()
        .map(parse_sequence)
        .try_fold(0, |acc, sequence| {
            let sequence = sequence?;
            let next_in_sequence = predict_next(&sequence)?;
            Ok(acc + next_in_sequence)
        })
}
