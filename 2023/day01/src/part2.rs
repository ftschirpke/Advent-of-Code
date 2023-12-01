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

#[derive(Debug, Clone)]
enum Digit {
    Ascii { idx: usize, digit: char },
    Word { idx: usize, word: &'static str },
}

impl PartialEq for Digit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Digit::Ascii { idx: idx1, .. }, Digit::Ascii { idx: idx2, .. }) => idx1 == idx2,
            (Digit::Word { idx: idx1, .. }, Digit::Word { idx: idx2, .. }) => idx1 == idx2,
            _ => false,
        }
    }
}

impl PartialOrd for Digit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Digit {}

impl Ord for Digit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Digit::Ascii { idx: idx1, .. }, Digit::Ascii { idx: idx2, .. }) => idx1.cmp(idx2),
            (Digit::Word { idx: idx1, .. }, Digit::Word { idx: idx2, .. }) => idx1.cmp(idx2),
            (Digit::Ascii { idx: idx1, .. }, Digit::Word { idx: idx2, .. }) => idx1.cmp(idx2),
            (Digit::Word { idx: idx1, .. }, Digit::Ascii { idx: idx2, .. }) => idx1.cmp(idx2),
        }
    }
}

impl Digit {
    fn value(self) -> Result<i32, AocError> {
        match self {
            Digit::Ascii { digit, .. } => char_to_digit(digit),
            Digit::Word { word, .. } => {
                let idx = DIGIT_NAMES.iter().position(|name| name == &word);
                let idx = idx.ok_or(AocError::LogicError(format!(
                    "Word {} not found in digit names",
                    word
                )))?;
                Ok(idx as i32)
            }
        }
    }
}

fn compare(digit: Option<Digit>, word: Option<Digit>, smaller: bool) -> Option<Digit> {
    match (digit, word) {
        (Some(digit), Some(word)) => {
            if smaller == (digit < word) {
                Some(digit)
            } else {
                Some(word)
            }
        }
        (Some(digit), None) => Some(digit),
        (None, Some(word)) => Some(word),
        (None, None) => None,
    }
}

fn get_calibration_value(line: &str) -> Result<i32, AocError> {
    let first_digit: Digit = {
        let first_digit: Option<Digit> = {
            let idx = line.chars().position(|c| c.is_ascii_digit());
            match idx {
                Some(idx) => Some(Digit::Ascii {
                    idx,
                    digit: line.chars().nth(idx).ok_or(AocError::LogicError(
                        "Cannot get found digit as char from iterator".to_string(),
                    ))?,
                }),
                None => None,
            }
        };
        let first_word: Option<Digit> = DIGIT_NAMES
            .iter()
            .filter_map(|name| {
                Some(Digit::Word {
                    idx: line.find(name)?,
                    word: name,
                })
            })
            .min();
        compare(first_digit, first_word, true).ok_or(AocError::LogicError(format!(
            "No digit found in line {}",
            line
        )))?
    };

    let last_digit: Digit = {
        let last_digit: Option<Digit> = {
            let reverse_idx = line.chars().rev().position(|c| c.is_ascii_digit());
            match reverse_idx {
                Some(idx) => Some(Digit::Ascii {
                    idx: line.len() - 1 - idx,
                    digit: line.chars().rev().nth(idx).ok_or(AocError::LogicError(
                        "Cannot get found digit as char from iterator".to_string(),
                    ))?,
                }),
                None => None,
            }
        };
        let last_word: Option<Digit> = DIGIT_NAMES
            .iter()
            .filter_map(|name| {
                Some(Digit::Word {
                    idx: line.rfind(name)?,
                    word: name,
                })
            })
            .max();
        compare(last_digit, last_word, false).ok_or(AocError::LogicError(format!(
            "No digit found in line {}",
            line
        )))?
    };

    let first_digit = first_digit.value()?;
    let last_digit = last_digit.value()?;

    Ok(first_digit * 10 + last_digit)
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    input
        .lines()
        .try_fold(0i32, |acc, line| Ok(acc + get_calibration_value(line)?))
}
