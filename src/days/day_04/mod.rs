mod digits;
mod exercise_1;
mod exercise_2;
mod passport;

use std::fs::File;
use std::io::prelude::*;

use passport::Passport;

pub fn solve(input_file: &mut File) {
    let passports = parse_input(input_file);

    println!("Exercise 1");
    exercise_1::solve(&passports);

    println!("Exercise 2");
    exercise_2::solve(&passports);
}

fn parse_input(input_file: &mut File) -> Vec<Passport> {
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();
    input_string
        .split("\n\n")
        .map(|raw| Passport::from_str(raw))
        .collect()
}
