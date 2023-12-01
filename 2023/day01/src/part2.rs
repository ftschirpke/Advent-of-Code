use aoclib::AocError;

static DIGIT_NAMES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn char_to_digit(c: char) -> Result<i32, AocError> {
    let val = c
        .to_digit(10)
        .ok_or(AocError::LogicError(format!("{} is not a digit", c)))?;
    let val = val as i32;
    Ok(val)
}

fn get_calibration_value(line: &str) -> Result<i32, AocError> {
    let mut digits = line.chars().enumerate().filter_map(|(i, c)| {
        if c.is_ascii_digit() {
            char_to_digit(c).ok()
        } else {
            let slice = &line[i..];
            for (val, name) in DIGIT_NAMES.iter().enumerate() {
                if slice.starts_with(name) {
                    return Some(val as i32);
                }
            }
            None
        }
    });
    let first_digit = digits
        .next()
        .ok_or(AocError::LogicError(format!("No digits found in {}", line)))?;
    let last_digit = digits.last().unwrap_or(first_digit);
    Ok(first_digit * 10 + last_digit)
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    input
        .lines()
        .try_fold(0i32, |acc, line| Ok(acc + get_calibration_value(line)?))
}
