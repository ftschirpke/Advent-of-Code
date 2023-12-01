use std::env;
use std::fs;

fn parse_args(args: Vec<String>) -> Result<String, String> {
    match args.len() {
        1 => Err(String::from("No arguments provided")),
        2 => Ok(args[1].clone()),
        _ => Err(String::from("Too many arguments provided")),
    }
}

#[derive(Clone)]
enum RPSType {
    Rock,
    Paper,
    Scissors,
}

struct RoundStrategy {
    enemy: RPSType,
    my: RPSType,
}

fn enemy_from_str(enemy_str: &str) -> RPSType {
    match enemy_str {
        "A" => RPSType::Rock,
        "B" => RPSType::Paper,
        "C" => RPSType::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn me_from_str(my_str: &str) -> RPSType {
    match my_str {
        "X" => RPSType::Rock,
        "Y" => RPSType::Paper,
        "Z" => RPSType::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn wanted_result_from_str(wanted_result_str: &str) -> RPSResult {
    match wanted_result_str {
        "X" => RPSResult::Loss,
        "Y" => RPSResult::Draw,
        "Z" => RPSResult::Win,
        _ => panic!("Invalid input"),
    }
}

enum RPSResult {
    Win,
    Draw,
    Loss,
}

fn result_score(result: &RPSResult) -> i32 {
    match result {
        RPSResult::Win => 6,
        RPSResult::Draw => 3,
        RPSResult::Loss => 0,
    }
}

fn choice_score(choice: &RPSType) -> i32 {
    match choice {
        RPSType::Rock => 1,
        RPSType::Paper => 2,
        RPSType::Scissors => 3,
    }
}

fn result(enemy: &RPSType, me: &RPSType) -> RPSResult {
    match enemy {
        RPSType::Rock => match me {
            RPSType::Rock => RPSResult::Draw,
            RPSType::Paper => RPSResult::Win,
            RPSType::Scissors => RPSResult::Loss,
        },
        RPSType::Paper => match me {
            RPSType::Rock => RPSResult::Loss,
            RPSType::Paper => RPSResult::Draw,
            RPSType::Scissors => RPSResult::Win,
        },
        RPSType::Scissors => match me {
            RPSType::Rock => RPSResult::Win,
            RPSType::Paper => RPSResult::Loss,
            RPSType::Scissors => RPSResult::Draw,
        },
    }
}

fn choice_for_result(enemy: &RPSType, wanted_result: &RPSResult) -> RPSType {
    match wanted_result {
        RPSResult::Loss => match enemy {
            RPSType::Rock => RPSType::Scissors,
            RPSType::Paper => RPSType::Rock,
            RPSType::Scissors => RPSType::Paper,
        },
        RPSResult::Draw => enemy.clone(),
        RPSResult::Win => match enemy {
            RPSType::Rock => RPSType::Paper,
            RPSType::Paper => RPSType::Scissors,
            RPSType::Scissors => RPSType::Rock,
        },
    }
}

fn score(strategy: RoundStrategy) -> i32 {
    result_score(&result(&strategy.enemy, &strategy.my)) + choice_score(&strategy.my)
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
    let round_pairs = contents.lines().map(|line| {
        let pair: Vec<_> = line.split_whitespace().collect();
        match pair.len() {
            2 => (pair[0], pair[1]),
            _ => panic!("Invalid input"),
        }
    });

    // PART ONE
    // let round_strategies = round_pairs.map(|(enemy_str, my_char)| RoundStrategy {
    //     enemy: enemy_from_str(enemy_str),
    //     my: me_from_str(my_char),
    // });

    // PART TWO
    let round_strategies = round_pairs.map(|(enemy_str, wanted_result_str)| {
        let enemy = enemy_from_str(enemy_str);
        let my = choice_for_result(&enemy, &wanted_result_from_str(wanted_result_str));
        RoundStrategy { enemy, my }
    });

    // BOTH
    let score_for_strategy: i32 = round_strategies.map(score).sum();
    println!(
        "With the given strategy, I would have a score of {} at the end of the tournament.",
        score_for_strategy
    );
}
