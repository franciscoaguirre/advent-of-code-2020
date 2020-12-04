mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input_file: File) {
    let expense_report = parse_input(input_file);

    println!("Exercise 1: ");
    exercise_1::solve(&expense_report);

    println!("Exercise 2: ");
    exercise_2::solve(&expense_report);
}

fn parse_input(input_file: File) -> Vec<u32> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}
