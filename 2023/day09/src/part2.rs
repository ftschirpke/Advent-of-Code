use std::collections::VecDeque;

use aoclib::AocError;

use crate::part1::{differences_in_sequences, parse_sequence};

enum SequenceInfo {
    PreviousItem(i32),
    FirstItem(i32),
}

fn try_predict_previous(sequence: &[i32]) -> Result<SequenceInfo, AocError> {
    let mut iter = sequence.iter();
    let first = iter.next().ok_or(AocError::LogicError(
        "Cannot predict previous in empty sequence".to_string(),
    ))?;
    if iter.all(|&x| x == *first) {
        Ok(SequenceInfo::PreviousItem(*first))
    } else {
        Ok(SequenceInfo::FirstItem(*first))
    }
}

pub fn predict_next(sequence: &[i32]) -> Result<i32, AocError> {
    let mut first_elements_stack = {
        let first_element = sequence.first().ok_or(AocError::LogicError(
            "Cannot predict previous in empty sequence".to_string(),
        ))?;
        VecDeque::from([*first_element])
    };
    let mut differences = differences_in_sequences(sequence)?;
    loop {
        match try_predict_previous(&differences)? {
            SequenceInfo::PreviousItem(prediction) => {
                let prediction_for_original = first_elements_stack
                    .into_iter()
                    .rev()
                    .fold(prediction, |acc, first| first - acc);
                return Ok(prediction_for_original);
            }
            SequenceInfo::FirstItem(first_item) => {
                first_elements_stack.push_back(first_item);
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
            let previous_in_sequence = predict_next(&sequence)?;
            Ok(acc + previous_in_sequence)
        })
}
