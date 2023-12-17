use std::iter::repeat;

use aoclib::AocError;

use crate::part1::{parse_row, DamagedRow, SpringStatus};

impl DamagedRow {
    pub fn unfolded(&self) -> Self {
        let mut row = vec![SpringStatus::Unknown; self.row.len() * 5 + 4];
        row[0..self.row.len()].copy_from_slice(&self.row);
        row[self.row.len() + 1..2 * self.row.len() + 1].copy_from_slice(&self.row);
        row[2 * self.row.len() + 2..3 * self.row.len() + 2].copy_from_slice(&self.row);
        row[3 * self.row.len() + 3..4 * self.row.len() + 3].copy_from_slice(&self.row);
        row[4 * self.row.len() + 4..5 * self.row.len() + 4].copy_from_slice(&self.row);

        let contiguous_groups = repeat(self.contiguous_groups.iter())
            .take(5)
            .flatten()
            .copied()
            .collect();

        Self::new(row, contiguous_groups)
    }
}

pub fn solve_row(input: &str) -> Result<usize, AocError> {
    let original_row = parse_row(input)?;
    let unfolded_row = original_row.unfolded();
    let solutions = unfolded_row.count_solutions();
    Ok(solutions)
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    input.lines().try_fold(0, |acc, line| {
        let row = solve_row(line)?;
        Ok(acc + row)
    })
}
