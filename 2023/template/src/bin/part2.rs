use aoclib::AocError;

use template::part2::process;

fn main() -> Result<(), AocError> {
    let input = include_str!("../../input2.txt");
    let output = process(input)?;
    println!("=== Part 2 Result ===");
    println!("{}", output);
    Ok(())
}
