mod datastructures;
mod parsing;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::parsing;
    use crate::part1;
    use crate::part2;
    use aoclib::AocError;

    #[test]
    fn test_part1() -> Result<(), AocError> {
        let input = include_str!("../test_input1.txt");
        if input.is_empty() {
            return Err(AocError::ParseError(
                "Input file for test 1 is empty".to_string(),
            ));
        }
        let output = part1::process(input)?;
        let expected_output = 19114;
        assert_eq!(expected_output, output);
        Ok(())
    }

    #[test]
    fn test_part1_by_line() -> Result<(), AocError> {
        let input = include_str!("../test_input1.txt");
        if input.is_empty() {
            return Err(AocError::ParseError(
                "Input file for test 1 is empty".to_string(),
            ));
        }
        let lines_accepted = [true, false, true, false, true];
        let (workflows, parts) = parsing::parse_input(input)?;
        lines_accepted
            .into_iter()
            .zip(parts)
            .for_each(|(should_get_accepted, part)| {
                println!("part: {:?}", part);
                assert_eq!(
                    should_get_accepted,
                    part1::do_workflows_accept_part(&workflows, &part)
                );
            });
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), AocError> {
        let input = include_str!("../test_input2.txt");
        if input.is_empty() {
            return Err(AocError::ParseError(
                "Input file for test 2 is empty".to_string(),
            ));
        }
        let output = part2::process(input)?;
        let expected_output = -1;
        todo!("expected output part 2");
        assert_eq!(expected_output, output);
        Ok(())
    }
}
