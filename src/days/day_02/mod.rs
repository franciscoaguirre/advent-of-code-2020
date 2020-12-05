mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

type PasswordWithRestrictions = (u32, u32, char, String);

pub fn solve(input_file: File) {
    let passwords_with_restrictions = parse_input(input_file);

    println!("Exercise 1: ");
    exercise_1::solve(&passwords_with_restrictions);

    println!("Exercise 2: ");
    exercise_2::solve(&passwords_with_restrictions);
}

fn parse_input(input_file: File) -> Vec<PasswordWithRestrictions> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let index_dash = line.find("-").unwrap();
            let index_first_space = line.find(" ").unwrap();

            let min_appearances: u32 = *&line[..index_dash].parse::<u32>().unwrap();
            let max_appearances: u32 = *&line[index_dash + 1..index_first_space]
                .parse::<u32>()
                .unwrap();
            let letter = line.chars().nth(index_first_space + 1).unwrap();
            let password = String::from(&line[index_first_space + 4..line.len()]);

            (min_appearances, max_appearances, letter, password)
        })
        .collect()
}
