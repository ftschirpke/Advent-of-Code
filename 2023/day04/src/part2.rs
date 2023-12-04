use std::collections::HashMap;

use aoclib::AocError;

use crate::part1::{number_of_matches, parse_card};

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let cards = input.lines().map(parse_card);
    let mut copies_won: HashMap<i32, i32> = HashMap::new();
    let mut number_of_cards = 0;
    for card in cards {
        number_of_cards += 1;
        let card = card?;
        let number_of_copies = *copies_won.get(&card.id).unwrap_or(&0) + 1;
        let matching_numbers = number_of_matches(&card);
        (1..=matching_numbers).for_each(|n| {
            copies_won
                .entry(card.id + n)
                .and_modify(|e| *e += number_of_copies)
                .or_insert(number_of_copies);
        });
    }
    let number_of_copies_won = copies_won.values().sum::<i32>();
    Ok(number_of_cards + number_of_copies_won)
}
