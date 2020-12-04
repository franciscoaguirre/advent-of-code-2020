mod day_01;

use std::fs::File;

pub fn solve_day(day: u32) {
    let input_file = File::open(format!("inputs/day{:02}.txt", day)).expect("Input file not found");

    match day {
        1 => day_01::solve(input_file),
        _ => panic!("No solution found for that day!"),
    };
}
