pub mod grid;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use aoclib::AocError;

    #[test]
    fn test_part1a() -> Result<(), AocError> {
        let input = include_str!("../test_input1a.txt");
        let output = part1::process(input)?;
        let expected_output = 4;
        assert_eq!(expected_output, output);
        Ok(())
    }

    #[test]
    fn test_part1b() -> Result<(), AocError> {
        let input = include_str!("../test_input1b.txt");
        let output = part1::process(input)?;
        let expected_output = 8;
        assert_eq!(expected_output, output);
        Ok(())
    }

    #[test]
    fn test_part2a() -> Result<(), AocError> {
        let input = include_str!("../test_input2a.txt");
        let output = part2::process(input)?;
        let expected_output = 4;
        assert_eq!(expected_output, output);
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<(), AocError> {
        let input = include_str!("../test_input2b.txt");
        let output = part2::process(input)?;
        let expected_output = 8;
        assert_eq!(expected_output, output);
        Ok(())
    }

    #[test]
    fn test_part2c() -> Result<(), AocError> {
        let input = include_str!("../test_input2c.txt");
        let output = part2::process(input)?;
        let expected_output = 10;
        assert_eq!(expected_output, output);
        Ok(())
    }
}
