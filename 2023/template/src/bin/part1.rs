use aoclib::AocError;

use template::part1::process;

fn main() -> Result<(), AocError> {
    let input = include_str!("../../input.txt");
    let output = process(input)?;
    println!("=== Part 1 Result ===");
    println!("{}", output);
    Ok(())
}
