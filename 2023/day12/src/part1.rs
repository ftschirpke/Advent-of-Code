use std::collections::VecDeque;

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
    row: Vec<SpringStatus>,
    contiguous_groups: Vec<u32>,
}

impl DamagedRow {
    pub fn new(row: Vec<SpringStatus>, contiguous_groups: Vec<u32>) -> Self {
        Self {
            row,
            contiguous_groups,
        }
    }

    pub fn calculate_solutions(&self) -> Vec<RowSolution> {
        let mut incomplete_solutions: VecDeque<RowSolution> = VecDeque::new();
        let mut complete_solutions: Vec<RowSolution> = Vec::new();
        incomplete_solutions.push_back(self.into());
        while let Some(mut solution) = incomplete_solutions.pop_back() {
            if solution.is_complete() {
                complete_solutions.push(solution);
            } else {
                if let Some(operational_option) = solution.with_next_damaged() {
                    if operational_option.is_valid() {
                        incomplete_solutions.push_back(operational_option);
                    }
                }
                if solution.set_next_operational() && solution.is_valid() {
                    incomplete_solutions.push_back(solution);
                }
            }
        }
        complete_solutions
    }
}

#[derive(Debug, Clone)]
pub struct RowSolution<'a> {
    original: &'a DamagedRow,
    solution: Vec<SpringStatus>,
}

impl<'a> From<&'a DamagedRow> for RowSolution<'a> {
    fn from(row: &'a DamagedRow) -> Self {
        let mut solution = Self {
            original: row,
            solution: row.row.clone(),
        };
        solution.set_implied_values();
        solution
    }
}

impl<'a> RowSolution<'a> {
    pub fn set_next_operational(&mut self) -> bool {
        let next_unknown = self
            .solution
            .iter_mut()
            .find(|status| **status == SpringStatus::Unknown);
        if let Some(next_unknown) = next_unknown {
            *next_unknown = SpringStatus::Operational;
            self.set_implied_values();
            true
        } else {
            false
        }
    }

    pub fn with_next_operational(&self) -> Option<Self> {
        let mut solution = self.clone();
        let changed = solution.set_next_operational();
        if changed {
            Some(solution)
        } else {
            None
        }
    }

    pub fn set_next_damaged(&mut self) -> bool {
        let next_unknown = self
            .solution
            .iter_mut()
            .find(|status| **status == SpringStatus::Unknown);
        if let Some(next_unknown) = next_unknown {
            *next_unknown = SpringStatus::Damaged;
            self.set_implied_values();
            true
        } else {
            false
        }
    }

    pub fn with_next_damaged(&self) -> Option<Self> {
        let mut solution = self.clone();
        let changed = solution.set_next_damaged();
        if changed {
            Some(solution)
        } else {
            None
        }
    }

    fn set_implied_values(&mut self) {
        let mut index = 0;
        let mut next_group = 0;
        while index < self.solution.len() {
            match self.solution[index] {
                SpringStatus::Operational => index += 1,
                SpringStatus::Damaged => {
                    let group_size = self.original.contiguous_groups[next_group];
                    let group_end = (index + group_size as usize).min(self.solution.len());
                    for i in index + 1..group_end {
                        self.solution[i] = SpringStatus::Damaged;
                    }
                    index = group_end;
                    next_group += 1;
                    if next_group >= self.original.contiguous_groups.len() {
                        for i in index..self.solution.len() {
                            self.solution[i] = SpringStatus::Operational;
                        }
                        index = self.solution.len();
                    } else if group_end < self.solution.len() {
                        self.solution[group_end] = SpringStatus::Operational;
                        index += 1;
                    }
                }
                SpringStatus::Unknown => break,
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut damaged_count = 0;
        let mut next_group = 0;
        let mut in_group: bool = false;
        for (original, solution) in self.original.row.iter().zip(self.solution.iter()) {
            match solution {
                SpringStatus::Damaged => {
                    if *original == SpringStatus::Operational {
                        return false;
                    }
                    if next_group >= self.original.contiguous_groups.len() {
                        return false;
                    }
                    damaged_count += 1;
                    in_group = true;
                }
                SpringStatus::Operational => {
                    if *original == SpringStatus::Damaged {
                        return false;
                    }
                    if in_group {
                        next_group += 1;
                        in_group = false;
                    }
                }
                SpringStatus::Unknown => {
                    if *original != SpringStatus::Unknown {
                        return false;
                    }
                }
            }
        }
        damaged_count
            <= self
                .original
                .contiguous_groups
                .iter()
                .fold(0, |acc, group| acc + *group)
    }

    fn is_complete(&self) -> bool {
        let no_unknowns_left = !self
            .solution
            .iter()
            .any(|status| *status == SpringStatus::Unknown);
        let mut groups = vec![0; self.original.contiguous_groups.len()];
        let mut in_group = false;
        let mut next_group = 0;
        for status in self.solution.iter() {
            match status {
                SpringStatus::Damaged => {
                    if !in_group {
                        in_group = true;
                    }
                    groups[next_group] += 1;
                }
                SpringStatus::Operational => {
                    if in_group {
                        in_group = false;
                        next_group += 1;
                    }
                }
                SpringStatus::Unknown => {}
            }
        }
        no_unknowns_left && groups == self.original.contiguous_groups
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
    let solutions = original_row.calculate_solutions();
    Ok(solutions.len() as u32)
}

pub fn process(input: &'static str) -> Result<u32, AocError> {
    input.lines().try_fold(0, |acc, line| {
        let row = solve_row(line)?;
        Ok(acc + row)
    })
}
