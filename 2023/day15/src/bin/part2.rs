use aoclib::AocError;

use day15::part2::process;

fn main() -> Result<(), AocError> {
    let input = include_str!("../../input.txt");
    let output = process(input)?;
    println!("=== Part 2 Result ===");
    println!("{}", output);
    Ok(())
}
