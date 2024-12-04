use std::{
    fs::File,
    io::{self, BufRead},
};

use regex::{Captures, Regex};

#[derive(Debug)]
struct Multiplication {
    left: i32,
    right: i32,
}

// part one
pub fn part_one() {
    match process_input_file_part_one() {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(e) => eprintln!("Some error ocurred when processing the input: {}", e),
    }
}

fn process_input_file_part_one() -> io::Result<i32> {
    let filepath = "./src/day03/input.txt";
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    let result = reader
        .lines()
        .map_while(Result::ok)
        .flat_map(parse)
        .map(calculate_result)
        .sum();

    Ok(result)
}

fn parse(input: String) -> Vec<Multiplication> {
    let regex = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
    regex.captures_iter(&input).map(process_captures).collect()
}

fn calculate_result(multiplication: Multiplication) -> i32 {
    multiplication.left * multiplication.right
}

fn process_captures(captures: Captures<'_>) -> Multiplication {
    let left = captures
        .name("left")
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    let right = captures
        .name("right")
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
    Multiplication { left, right }
}

// part two
pub fn part_two() {
    let input = process_input_file_part_two();

    process_input(input);
}

fn process_input_file_part_two() -> io::Result<String> {
    let filepath = "./src/day03/input.txt";
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    let result = reader.lines().map_while(Result::ok).collect();

    Ok(result)
}

fn process_input(input: io::Result<String>) {
    match input {
        Ok(checked_input) => {
            let solution = part_two_logic(checked_input);
            println!("{:?}", solution);
        }
        Err(e) => eprintln!("Some error ocurred when processing the input: {}", e),
    }
}

fn part_two_logic(input: String) -> i32 {
    let cleaned_input = clean_input(input);
    parse(cleaned_input).into_iter().map(calculate_result).sum()
}

fn clean_input(input: String) -> String {
    let re = Regex::new(r"(?s)don't\(\).*?(do\(\)|$)").unwrap();
    re.replace_all(&input, "").to_string()
}
