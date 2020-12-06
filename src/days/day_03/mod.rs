use std::fs::File;
use std::io::{BufRead, BufReader};

mod exercise_1;
mod exercise_2;

type TreePositions = Vec<usize>;

pub fn solve(input_file: File) {
    let input = parse_input(input_file);

    println!("Exercise 1: ");
    println!("Solution: {}", exercise_1::solve(&input));

    println!("Exercise 2: ");
    println!("Solution: {}", exercise_2::solve(&input));
}

fn parse_input(input_file: File) -> (usize, Vec<TreePositions>) {
    let mut pattern_width = 0;
    let tree_positions_per_row: Vec<TreePositions> = BufReader::new(input_file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            pattern_width = line.len();
            let mut tree_locations: TreePositions = Vec::new();
            for (index, character) in line.chars().enumerate() {
                if character == '#' {
                    tree_locations.push(index);
                }
            }
            tree_locations
        })
        .collect();
    (pattern_width, tree_positions_per_row)
}
