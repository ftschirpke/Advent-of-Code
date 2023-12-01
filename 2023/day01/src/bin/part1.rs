use aoclib::AocError;

use day01::part1::process;

fn main() -> Result<(), AocError> {
    let input = include_str!("../../input1.txt");
    let output = process(input)?;
    println!("=== Part 1 Result ===");
    println!("{}", output);
    Ok(())
}
