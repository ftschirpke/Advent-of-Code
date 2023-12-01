pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use aoclib::AocError;

    #[test]
    fn test_part1() -> Result<(), AocError> {
        let input = include_str!("../test_input1.txt");
        let output = part1::process(input)?;
        let expected_output = 142;
        assert_eq!(expected_output, output);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), AocError> {
        let input = include_str!("../test_input2.txt");
        let output = part2::process(input)?;
        let expected_output = 281;
        assert_eq!(expected_output, output);
        Ok(())
    }
}
