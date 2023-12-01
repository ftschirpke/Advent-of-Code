use aoclib::AocError;

fn char_to_digit(c: char) -> Result<i32, AocError> {
    let val = c
        .to_digit(10)
        .ok_or(AocError::LogicError(format!("{} is not a digit", c)))?;
    let val = val as i32;
    Ok(val)
}

fn get_calibration_value(line: &str) -> Result<i32, AocError> {
    let mut it = line.chars().filter(|c| c.is_ascii_digit());
    let first_digit = it
        .next()
        .ok_or(AocError::ParseError(String::from("No digits found")))?;
    let last_digit = it.last().unwrap_or(first_digit);
    let first_digit = char_to_digit(first_digit)?;
    let last_digit = char_to_digit(last_digit)?;
    Ok(first_digit * 10 + last_digit)
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    input
        .lines()
        .try_fold(0i32, |acc, line| Ok(acc + get_calibration_value(line)?))
}
