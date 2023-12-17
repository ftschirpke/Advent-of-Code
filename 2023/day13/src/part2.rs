use aoclib::AocError;

use bitvec::vec::BitVec;

use crate::part1::{parse_all, MirrorAxis, Terrain};

pub fn find_mirror_index(bitvecs: &[BitVec]) -> Option<usize> {
    let len = bitvecs.len();
    (0..len - 1).find(|idx| {
        let mut diff = 0;
        let before_indices = (0..=*idx).rev();
        let after_indices = (*idx + 1)..len;
        for (before_idx, after_idx) in before_indices.zip(after_indices) {
            let xor_result = bitvecs[before_idx].clone() ^ bitvecs[after_idx].clone();
            diff += xor_result.count_ones();
            if diff > 1 {
                return false;
            }
        }
        diff == 1
    })
}

pub fn find_mirror_axis(terrain: &Terrain) -> Option<MirrorAxis> {
    find_mirror_index(&terrain.cols)
        .map(MirrorAxis::Vertical)
        .or_else(|| find_mirror_index(&terrain.rows).map(MirrorAxis::Horizontal))
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let terrains = parse_all(input)?;
    let score = terrains
        .iter()
        .filter_map(find_mirror_axis)
        .map(|axis| match axis {
            MirrorAxis::Horizontal(idx) => (idx + 1) * 100,
            MirrorAxis::Vertical(idx) => idx + 1,
        })
        .sum();
    Ok(score)
}
