use aoclib::AocError;

use crate::part1::{adjust_galaxy_positions, galaxy_distance, parse_galaxy_positions};

pub fn general_process(
    input: &'static str,
    galaxy_expansion_factor: usize,
) -> Result<usize, AocError> {
    let galaxy_positions = parse_galaxy_positions(input)?;
    let galaxy_positions = adjust_galaxy_positions(galaxy_positions, galaxy_expansion_factor);
    let res = galaxy_positions
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy)| {
            galaxy_positions
                .iter()
                .skip(i + 1)
                .filter(move |other_galaxy| *other_galaxy != galaxy)
                .map(move |other_galaxy| galaxy_distance(galaxy, other_galaxy) as usize)
        })
        .sum();
    Ok(res)
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    general_process(input, 1000000)
}
