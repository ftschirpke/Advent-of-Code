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

    let section_strs = contents
        .lines()
        .map(|line| line.split(['-', ',']).collect::<Vec<_>>());
    let section_vals = section_strs.map(|strs| match strs[..] {
        [first_start, first_end, second_start, second_end] => (
            first_start.parse::<i32>().unwrap(),
            first_end.parse::<i32>().unwrap(),
            second_start.parse::<i32>().unwrap(),
            second_end.parse::<i32>().unwrap(),
        ),
        _ => panic!("Invalid input!"),
    });

    // PART ONE
    // let number_of_fully_contained = section_vals
    //     .filter(|(a, b, c, d)| {
    //         let x = (c - a).abs() + (b - d).abs();
    //         let y = (c - a + b - d).abs();
    //         x == y
    //     })
    //     .count();
    // println!(
    //     "Number of fully contained ranges: {}",
    //     number_of_fully_contained
    // );
    // PART TWO
    let overlapped_sections = section_vals.filter(|(a, b, c, d)| a <= d && c <= b).count();
    println!("Number of overlapped sections: {}", overlapped_sections);
}
