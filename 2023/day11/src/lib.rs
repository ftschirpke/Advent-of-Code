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
        let expected_output = 374;
        assert_eq!(expected_output, output);
        Ok(())
    }

    #[test]
    fn test_part2a() -> Result<(), AocError> {
        let input = include_str!("../test_input1.txt");
        let output = part2::general_process(input, 10)?;
        let expected_output = 1030;
        assert_eq!(expected_output, output);
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<(), AocError> {
        let input = include_str!("../test_input1.txt");
        let output = part2::general_process(input, 100)?;
        let expected_output = 8410;
        assert_eq!(expected_output, output);
        Ok(())
    }
}
