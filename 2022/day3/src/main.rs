use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_args(args: Vec<String>) -> Result<String, String> {
    match args.len() {
        1 => Err(String::from("No arguments provided")),
        2 => Ok(args[1].clone()),
        _ => Err(String::from("Too many arguments provided")),
    }
}

fn char_priority(c: char) -> i32 {
    match c.is_ascii_alphabetic() {
        true => {
            let mut dig = c.to_digit(36).unwrap();
            if c.is_uppercase() {
                dig += 26;
            }
            dig -= 9;
            dig as i32
        }
        false => panic!("Invalid input!"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_args(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        println!("Usage: cargo run <filename>");
        std::process::exit(1);
    });

    let contents = fs::read_to_string(&filename).unwrap_or_else(|err| {
        println!("Problem reading from file {}: {}", filename, err);
        std::process::exit(1);
    });

    assert_eq!(char_priority('a'), 1);
    assert_eq!(char_priority('Z'), 52);

    // PART ONE
    let duplicate_characters = contents.lines().map(|line| {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let first = first_half.chars().collect::<HashSet<char>>();
        let second = second_half.chars().collect::<HashSet<char>>();
        let chars_in_both: Vec<&char> = first.intersection(&second).collect();
        match chars_in_both.len() {
            1 => *chars_in_both[0],
            _ => panic!("Invalid input!"),
        }
    });
    let priority_sum = duplicate_characters.fold(0, |acc, c| acc + char_priority(c));
    println!("Sum of all priorities: {}", priority_sum);

    // PART TWO
    let lines: Vec<&str> = contents.lines().collect();
    let number_of_groups = lines.len() / 3;
    let mut group_badges: Vec<char> = Vec::new();
    for group_num in 0..number_of_groups {
        let idx = 3 * group_num;
        let mut first = lines[idx].chars().collect::<HashSet<char>>();
        let second = lines[idx + 1].chars().collect::<HashSet<char>>();
        let third = lines[idx + 2].chars().collect::<HashSet<char>>();
        first.retain(|c| second.contains(c) && third.contains(c));
        match first.len() {
            1 => group_badges.push(*first.iter().next().unwrap()),
            _ => panic!("Invalid input!"),
        };
    }
    let priority_sum = group_badges
        .iter()
        .fold(0, |acc, c| acc + char_priority(*c));
    println!("Sum of all badge priorities: {}", priority_sum);
}
