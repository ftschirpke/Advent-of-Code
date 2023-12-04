use aoclib::AocError;

use crate::part1::{is_adjacent, parse_schematic, PartKind, SchematicPart};

impl SchematicPart {
    fn is_potential_gear(&self) -> bool {
        matches!(self.kind, PartKind::Symbol { symbol: '*' })
    }
}

fn add_adjacent_numbers_to_vec(
    gear_part: &SchematicPart,
    row: &[SchematicPart],
    adjacent_numbers: &mut Vec<i32>,
) {
    row.iter()
        .filter(|part| is_adjacent(part.idx, part.kind.width(), gear_part.idx))
        .for_each(|part| {
            if let PartKind::Number { value, .. } = part.kind {
                adjacent_numbers.push(value);
            }
        });
}

fn gear_ratio(
    gear_part: &SchematicPart,
    neighbors: (Option<&SchematicPart>, Option<&SchematicPart>),
    prev_row: Option<&Vec<SchematicPart>>,
    next_row: Option<&Vec<SchematicPart>>,
) -> i32 {
    let mut adjacent_numbers: Vec<i32> = Vec::new();
    if let Some(prev) = neighbors.0 {
        if let PartKind::Number { value, .. } = prev.kind {
            adjacent_numbers.push(value);
        }
    }
    if let Some(next) = neighbors.1 {
        if let PartKind::Number { value, .. } = next.kind {
            adjacent_numbers.push(value);
        }
    }
    if let Some(prev_row) = prev_row {
        add_adjacent_numbers_to_vec(gear_part, prev_row, &mut adjacent_numbers);
    }
    if let Some(next_row) = next_row {
        add_adjacent_numbers_to_vec(gear_part, next_row, &mut adjacent_numbers);
    }
    match &adjacent_numbers[..] {
        [a, b] => a * b,
        _ => 0,
    }
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let schematic = parse_schematic(input)?;
    let mut gear_ratio_sum = 0;
    for (row_idx, row) in schematic.iter().enumerate() {
        let prev_row = if row_idx > 0 {
            Some(&schematic[row_idx - 1])
        } else {
            None
        };
        let next_row = if row_idx < schematic.len() - 1 {
            Some(&schematic[row_idx + 1])
        } else {
            None
        };
        for (i, part) in row
            .iter()
            .enumerate()
            .filter(|(_, part)| part.is_potential_gear())
        {
            let prev = if i > 0 { Some(&row[i - 1]) } else { None };
            let next = if i < row.len() - 1 {
                Some(&row[i + 1])
            } else {
                None
            };
            gear_ratio_sum += gear_ratio(part, (prev, next), prev_row, next_row);
        }
    }
    Ok(gear_ratio_sum)
}
