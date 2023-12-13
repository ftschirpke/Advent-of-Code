use aoclib::AocError;

use bitvec::vec::BitVec;

use crate::part1::{find_mirror_axis, parse_all, MirrorAxis, Terrain};

struct FoundSmudge {
    smudge_outer_idx: usize,
    smudge_inner_idx: usize,
    new_mirror_axis_idx: usize,
}

fn find_smudge(bitvecs: &[BitVec]) -> Option<FoundSmudge> {
    let len = bitvecs.len();
    if len < 2 {
        return None;
    }
    let mut xor_result = bitvecs[0].clone() ^ bitvecs[1].clone();
    let mut range_start = 0;
    let mut range_end = 1;
    loop {
        if range_end < len - 2 {
            range_end += 1;
            xor_result ^= bitvecs[range_end].clone();
            range_end += 1;
            xor_result ^= bitvecs[range_end].clone();
        } else if range_start < len - 2 {
            xor_result ^= bitvecs[range_start].clone();
            range_start += 1;
            xor_result ^= bitvecs[range_start].clone();
            range_start += 1;
        } else {
            break;
        }
        if xor_result.count_ones() == 1 {
            // smudge is somewhere in the range
            let mirrored_count = (range_end - range_start + 1) / 2;
            let new_mirror_axis_idx = range_start + mirrored_count - 1;
            let mut idx = 0;
            println!(
                "Found new mirrox axis?! {}, {} ({})",
                range_start, range_end, new_mirror_axis_idx
            );
            bitvecs.iter().enumerate().for_each(|(i, v)| {
                println!("[{}]: {}", i, v);
            });
            println!("xor_result: {}", xor_result);
            loop {
                let before_idx = new_mirror_axis_idx - idx;
                let after_idx = new_mirror_axis_idx + 1 + idx;
                let diff = bitvecs[before_idx].clone() ^ bitvecs[after_idx].clone();
                let ones = diff.count_ones();
                match ones {
                    0 => {}
                    1 => {
                        let smudge_outer_idx = before_idx;
                        let smudge_inner_idx = diff.first_one().unwrap();
                        return Some(FoundSmudge {
                            smudge_outer_idx,
                            smudge_inner_idx,
                            new_mirror_axis_idx,
                        });
                    }
                    _ => {
                        break;
                    }
                }
                if before_idx == range_start || after_idx == range_end {
                    break;
                } else {
                    idx += 1;
                }
            }
        }
    }
    None
}

fn fix_smudge(terrain: Terrain) -> Result<(Terrain, MirrorAxis), AocError> {
    let original_mirror_axis = find_mirror_axis(&terrain).ok_or_else(|| {
        AocError::LogicError(format!(
            "Could not find mirror axis for terrain: {:?}",
            terrain
        ))
    })?;
    let (smudge_row, smudge_col, new_mirror_axis) = if let Some(smudge) = find_smudge(&terrain.rows)
    {
        (
            smudge.smudge_outer_idx,
            smudge.smudge_inner_idx,
            MirrorAxis::Horizontal(smudge.new_mirror_axis_idx),
        )
    } else if let Some(smudge) = find_smudge(&terrain.cols) {
        (
            smudge.smudge_inner_idx,
            smudge.smudge_outer_idx,
            MirrorAxis::Vertical(smudge.new_mirror_axis_idx),
        )
    } else {
        return Err(AocError::LogicError(format!(
            "Could not find smudge for terrain: {:?}",
            terrain
        )));
    };
    let smudge_before = terrain.rows[smudge_row][smudge_col];
    let mut terrain = terrain;
    terrain.rows[smudge_row].set(smudge_col, !smudge_before);
    terrain.cols[smudge_col].set(smudge_row, !smudge_before);
    if original_mirror_axis == new_mirror_axis {
        return Err(AocError::LogicError(format!(
            "Smudge was not fixed for terrain: {:?}",
            terrain
        )));
    }
    Ok((terrain, new_mirror_axis))
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let terrains = parse_all(input)?;
    let score = terrains
        .into_iter()
        .map(fix_smudge)
        .try_fold(0usize, |acc, fixed_res| {
            let (_, new_axis) = fixed_res?;
            let axis_score = match new_axis {
                MirrorAxis::Horizontal(idx) => 100 * (idx + 1),
                MirrorAxis::Vertical(idx) => idx + 1,
            };
            Ok(acc + axis_score)
        });
    score
}
