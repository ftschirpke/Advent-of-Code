use std::collections::{HashMap, VecDeque};

use aoclib::AocError;
use nom::{
    branch::alt,
    character::complete::{char as char_parser, space1, u32 as u32_parser},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DamagedRow {
    pub row: Vec<SpringStatus>,
    pub contiguous_groups: Vec<u32>,
}

impl DamagedRow {
    pub fn new(row: Vec<SpringStatus>, contiguous_groups: Vec<u32>) -> Self {
        Self {
            row,
            contiguous_groups,
        }
    }

    /// Checks whether given the data we have about the contiguous groups of springs
    /// it is possible to have the next group start at the current row index
    fn group_is_possible_from_index(&self, row_idx: usize, cg_idx: usize) -> bool {
        let current_group = self.contiguous_groups[cg_idx] as usize;
        let left_in_row = self.row.len() - row_idx;
        if current_group > left_in_row {
            return false;
        }
        let operational_in_supposed_group =
            self.row[row_idx..(row_idx + current_group)].contains(&SpringStatus::Operational);
        if operational_in_supposed_group {
            return false;
        }
        current_group == left_in_row // group fits perfectly 
            || self.row[row_idx + current_group] != SpringStatus::Damaged // group ends as intended
    }

    /// Checks whether the current indices are at then end of their corresponding vectors and
    /// additionally returns a count if this is the case
    /// this count is one if solutions ending up with these indices should be counted as a solution
    /// and zero if that is not the case and the solution should be discarded as invalid
    fn is_at_end_with_count(&self, row_idx: usize, cg_idx: usize) -> Option<usize> {
        let at_row_end = row_idx >= self.row.len();
        let at_cg_end = cg_idx >= self.contiguous_groups.len();
        if at_row_end && at_cg_end {
            Some(1)
        } else if at_row_end {
            Some(0)
        } else if at_cg_end {
            if self.row[row_idx..].contains(&SpringStatus::Damaged) {
                Some(0)
            } else {
                Some(1)
            }
        } else {
            None
        }
    }

    /// Returns which indices need to be checked next if the current spring is damaged
    fn next_to_check_if_damaged(&self, row_idx: usize, cg_idx: usize) -> Option<(usize, usize)> {
        match self.row[row_idx] {
            SpringStatus::Damaged | SpringStatus::Unknown => {
                // when the spring is or might be damaged, we need to check if that is possible
                // from the data we have about the contiguous groups of damaged springs
                if self.group_is_possible_from_index(row_idx, cg_idx) {
                    let current_group = self.contiguous_groups[cg_idx] as usize;
                    let next_row_idx = row_idx + current_group + 1;
                    Some((next_row_idx, cg_idx + 1))
                } else {
                    None
                }
            }
            // spring cannot be damaged when we know that it is operational
            SpringStatus::Operational => None,
        }
    }

    /// Returns which indices need to be checked next if the current index is operational
    fn next_to_check_if_operational(
        &self,
        row_idx: usize,
        cg_idx: usize,
    ) -> Option<(usize, usize)> {
        match self.row[row_idx] {
            // spring cannot be operational when we know that it is damaged
            SpringStatus::Damaged => None,
            SpringStatus::Operational | SpringStatus::Unknown => {
                // when the spring is or might be operational, we need to continue checking for
                // solutions at the next spring
                Some((row_idx + 1, cg_idx))
            }
        }
    }

    pub fn count_solutions(&self) -> usize {
        let mut counts_for_checked_approaches: HashMap<(usize, usize), usize> = HashMap::new();
        let mut approaches_to_check: VecDeque<(usize, usize)> = VecDeque::new();
        approaches_to_check.push_back((0, 0));
        while !approaches_to_check.is_empty() {
            let approach = approaches_to_check.back().unwrap();
            if counts_for_checked_approaches.contains_key(approach) {
                approaches_to_check.pop_back();
            } else {
                let (row_idx, cg_idx) = *approach;
                if let Some(count) = self.is_at_end_with_count(row_idx, cg_idx) {
                    counts_for_checked_approaches.insert((row_idx, cg_idx), count);
                    approaches_to_check.pop_back();
                } else {
                    let count_if_operational = if let Some(approach) =
                        self.next_to_check_if_operational(row_idx, cg_idx)
                    {
                        if let Some(&count) = counts_for_checked_approaches.get(&approach) {
                            Some(count)
                        } else {
                            approaches_to_check.push_back(approach);
                            None
                        }
                    } else {
                        Some(0)
                    };
                    let count_if_damaged =
                        if let Some(approach) = self.next_to_check_if_damaged(row_idx, cg_idx) {
                            if let Some(&count) = counts_for_checked_approaches.get(&approach) {
                                Some(count)
                            } else {
                                approaches_to_check.push_back(approach);
                                None
                            }
                        } else {
                            Some(0)
                        };
                    if let (Some(a), Some(b)) = (count_if_operational, count_if_damaged) {
                        counts_for_checked_approaches.insert((row_idx, cg_idx), a + b);
                        approaches_to_check.pop_back();
                    }
                }
            }
        }
        *counts_for_checked_approaches.get(&(0, 0)).unwrap()
    }
}

fn status_parser(input: &str) -> IResult<&str, SpringStatus> {
    alt((
        value(SpringStatus::Operational, char_parser('.')),
        value(SpringStatus::Damaged, char_parser('#')),
        value(SpringStatus::Unknown, char_parser('?')),
    ))(input)
}

fn parse_row_data(input: &str) -> IResult<&str, Vec<SpringStatus>> {
    many1(status_parser)(input)
}

fn parse_contiguous_groups(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, groups) = separated_list1(char_parser(','), u32_parser)(input)?;
    Ok((input, groups))
}

pub fn parse_row(input: &str) -> Result<DamagedRow, AocError> {
    let (rest, (row, contiguous_groups)) =
        separated_pair(parse_row_data, space1, parse_contiguous_groups)(input)?;
    if rest.is_empty() {
        Ok(DamagedRow::new(row, contiguous_groups))
    } else {
        Err(AocError::ParseError(format!(
            "Parsing Row left rest: '{}'",
            rest
        )))
    }
}

pub fn solve_row(input: &str) -> Result<u32, AocError> {
    let original_row = parse_row(input)?;
    let solutions = original_row.count_solutions();
    Ok(solutions as u32)
}

pub fn process(input: &'static str) -> Result<u32, AocError> {
    input.lines().try_fold(0, |acc, line| {
        let row = solve_row(line)?;
        Ok(acc + row)
    })
}
