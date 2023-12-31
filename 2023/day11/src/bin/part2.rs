use aoclib::AocError;

use day11::part2::process;

fn main() -> Result<(), AocError> {
    let input = include_str!("../../input.txt");
    if input.is_empty() {
        return Err(AocError::ParseError("Input is empty".to_string()));
    }
    let output = process(input)?;
    println!("=== Part 2 Result ===");
    println!("{}", output);
    Ok(())
}
