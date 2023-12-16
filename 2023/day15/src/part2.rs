use aoclib::AocError;

use crate::part1::{char_hash, parse_input, EntryEnding};

#[derive(Debug, Copy, Clone)]
pub struct Lens {
    label: &'static str,
    focal_length: u8,
}

pub fn process(input: &'static str) -> Result<u32, AocError> {
    let entries = parse_input(input)?;
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    entries.into_iter().for_each(|entry| {
        let label = entry.character_sequence.chars().fold(0u32, char_hash) as usize;
        let current_box = &mut boxes[label];
        let lens_index = current_box
            .iter()
            .position(|lens| lens.label == entry.character_sequence);
        match entry.ending {
            EntryEnding::Minus => {
                if let Some(lens_index) = lens_index {
                    current_box.remove(lens_index);
                }
            }
            EntryEnding::Equal(focal_length) => {
                let lens = Lens {
                    label: entry.character_sequence,
                    focal_length,
                };
                match lens_index {
                    Some(lens_index) => current_box[lens_index] = lens,
                    None => current_box.push(lens),
                }
            }
        }
    });
    let score = boxes
        .into_iter()
        .enumerate()
        .map(|(box_num, current_box)| -> u32 {
            (box_num + 1) as u32
                * current_box
                    .into_iter()
                    .enumerate()
                    .map(|(slot_num, lens)| (slot_num + 1) as u32 * lens.focal_length as u32)
                    .sum::<u32>()
        })
        .sum();
    Ok(score)
}
