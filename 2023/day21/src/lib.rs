pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use aoclib::grid::Grid;
    use aoclib::AocError;

    #[test]
    fn test_part1() -> Result<(), AocError> {
        let input = include_str!("../test_input1.txt");
        if input.is_empty() {
            return Err(AocError::ParseError(
                "Input file for test 1 is empty".to_string(),
            ));
        }
        let grid = Grid::parse_from(input, part1::parse_tile)?;
        let output = part1::count_garden_plots_for_fixed_step_amount(&grid, 6)?;
        let expected_output = 16;
        assert_eq!(expected_output, output);
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
