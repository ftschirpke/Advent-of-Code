use std::cmp::max;

use aoclib::AocError;

use crate::part1::{Game, Set};

impl Set {
    pub fn new() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }

    pub fn power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

impl Default for Set {
    fn default() -> Self {
        Self::new()
    }
}

fn game_power(line: &str) -> Result<i32, AocError> {
    let game = line.parse::<Game>()?;
    let minimum_set = game.sets.iter().fold(Set::new(), |acc, set| Set {
        red: max(acc.red, set.red),
        blue: max(acc.blue, set.blue),
        green: max(acc.green, set.green),
    });
    Ok(minimum_set.power())
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let mut sum_of_game_powers = 0;
    input.lines().try_for_each(|line| {
        sum_of_game_powers += game_power(line)?;
        Ok::<(), AocError>(())
    })?;
    Ok(sum_of_game_powers)
}
