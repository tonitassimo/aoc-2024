use std::{
    fs::File,
    io::{self, BufRead},
};

// common
fn process_input_file() -> io::Result<Vec<Vec<i32>>> {
    let filepath = "./src/day02/input.txt";
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);
    let mut input: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                let numbers: Vec<i32> = line_content
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                input.push(numbers);
            }
            Err(e) => return Err(e),
        }
    }

    Ok(input)
}

fn process_input(input: io::Result<Vec<Vec<i32>>>, logic_func: fn(Vec<Vec<i32>>) -> i32) {
    match input {
        Ok(entries) => {
            let solution = logic_func(entries);
            println!("{:?}", solution);
        }
        Err(e) => eprintln!("Some error ocurred when processing the input: {}", e),
    }
}

// part one
pub fn part_one() {
    let input = process_input_file();

    process_input(input, part_one_logic);
}

fn part_one_logic(entries: Vec<Vec<i32>>) -> i32 {
    entries
        .into_iter()
        .map(is_entry_safe)
        .filter(|safe| *safe)
        .count() as i32
}

fn is_entry_safe(entry: Vec<i32>) -> bool {
    let mut increasing = false;
    let mut decreasing = false;

    for window in entry.windows(2) {
        let (first, second) = (window[0], window[1]);
        match first.cmp(&second) {
            std::cmp::Ordering::Less => increasing = true,
            std::cmp::Ordering::Greater => decreasing = true,
            std::cmp::Ordering::Equal => {}
        }
        let difference = (first - second).abs();
        if !(1..=3).contains(&difference) {
            return false;
        }
    }

    increasing != decreasing
}

// part two
pub fn part_two() {
    let input = process_input_file();

    process_input(input, part_two_logic);
}

fn part_two_logic(entries: Vec<Vec<i32>>) -> i32 {
    entries
        .into_iter()
        .map(calculate_sub_sets)
        .map(is_any_entry_safe)
        .filter(|safe| *safe)
        .count() as i32
}

fn is_any_entry_safe(subsets: Vec<Vec<i32>>) -> bool {
    subsets.into_iter().map(is_entry_safe).any(|s| s)
}

fn calculate_sub_sets(entry: Vec<i32>) -> Vec<Vec<i32>> {
    entry
        .iter()
        .enumerate()
        .map(|(i, _)| [&entry[..i], &entry[i + 1..]].concat())
        .collect()
}
