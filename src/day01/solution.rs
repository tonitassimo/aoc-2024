use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn process_input_file() -> io::Result<(Vec<i32>, Vec<i32>)> {
    let filepath = "./src/day01/input.txt";
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                let numbers: Vec<i32> = line_content
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                left.push(numbers[0]);
                right.push(numbers[1]);
            }
            Err(e) => {
                return Err(e)
            }
        }
    }

    Ok((left, right))
}

pub fn part_one() {
    let input_result = process_input_file();

    match input_result {
        Ok((left, right)) => {
            let solution = part_one_logic(left, right);
            println!("{:?}", solution);
        }
        Err(e) => {
            eprintln!("Some error ocurred when processing the input: {}", e)
        }
    }
}

fn part_one_logic(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    let tuples = left.into_iter().zip(right);
    let result: i32 = tuples.map(|(left, right)| (left - right).abs()).sum();

    result
}

pub fn part_two() {
    let input_result = process_input_file();

    match input_result {
        Ok((left, right)) => {
            let solution = part_two_logic(left, right);
            println!("{:?}", solution);
        }
        Err(e) => {
            eprintln!("Some error ocurred when processing the input: {}", e)
        }
    }
}

fn part_two_logic(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let left_grouped = left.into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    let right_grouped = right.into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let mut sum: i32 = 0;
    for (key, value) in &left_grouped {
        sum += key * value * right_grouped.get(key).unwrap_or(&0)
    }

    sum
}
