use std::collections::BinaryHeap;
use std::env;
use std::fs;

fn parse_args(args: Vec<String>) -> Result<String, String> {
    match args.len() {
        1 => Err(String::from("No arguments provided")),
        2 => Ok(args[1].clone()),
        _ => Err(String::from("Too many arguments provided")),
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

    let elf_paragraphs = contents.split("\n\n");
    let calories_per_elf = elf_paragraphs.map(|calories_list| {
        calories_list
            .lines()
            .map(|calories_str| calories_str.parse::<i32>().unwrap_or(0))
            .sum()
    });

    // PART ONE
    // let max_calories = calories_per_elf.max().unwrap_or(0);

    // println!("maximum calories: {}", max_calories);

    // PART TWO
    let mut calories: BinaryHeap<i32> = calories_per_elf.collect();
    let first_three: i32 = (0..3).map(|_| calories.pop().unwrap_or(0)).sum();
    println!(
        "first three elfs together have carry {} calories",
        first_three
    );
}
