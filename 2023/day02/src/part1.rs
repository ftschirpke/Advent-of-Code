use std::str::FromStr;

use aoclib::AocError;

#[derive(Debug)]
pub struct Game {
    id: i32,
    pub sets: Vec<Set>,
}

#[derive(Debug)]
pub struct Set {
    pub red: i32,
    pub blue: i32,
    pub green: i32,
}

impl FromStr for Game {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("Game") {
            return Err(AocError::ParseError(format!(
                "invalid game: line does not start with 'Game': {:?}",
                s
            )));
        }
        let colon_idx = s.find(':').ok_or_else(|| {
            AocError::ParseError(format!("invalid game: no colon found in line {:?}", s))
        })?;
        let id = s[5..colon_idx].parse::<i32>()?;
        let set_strs = s[colon_idx + 1..].split(';').collect::<Vec<_>>();
        let sets = {
            let mut sets = Vec::with_capacity(set_strs.len());
            set_strs.iter().try_for_each(|set_str| {
                let parsed_set = set_str.parse::<Set>()?;
                sets.push(parsed_set);
                Ok::<(), AocError>(())
            })?;
            sets
        };
        Ok(Game { id, sets })
    }
}

impl FromStr for Set {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(", ").collect::<Vec<_>>();
        let parts = {
            let mut parts = parts
                .iter()
                .map(|part| part.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            parts.sort_by_key(|part| part[1]);
            parts
        };
        let red = &parts.iter().find(|part| part[1] == "red");
        let red = if let Some(red) = red {
            red[0].parse::<i32>()?
        } else {
            0
        };
        let green = &parts.iter().find(|part| part[1] == "green");
        let green = if let Some(green) = green {
            green[0].parse::<i32>()?
        } else {
            0
        };
        let blue = &parts.iter().find(|part| part[1] == "blue");
        let blue = if let Some(blue) = blue {
            blue[0].parse::<i32>()?
        } else {
            0
        };
        Ok(Set { red, green, blue })
    }
}

pub fn game_id_if_possible(line: &str) -> Result<Option<i32>, AocError> {
    let game = line.parse::<Game>()?;
    let possible = game
        .sets
        .iter()
        .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14);
    if possible {
        Ok(Some(game.id))
    } else {
        Ok(None)
    }
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    input.lines().try_fold(0i32, |count, line| {
        if let Some(id) = game_id_if_possible(line)? {
            Ok(count + id)
        } else {
            Ok(count)
        }
    })
}
