use super::exercise_1;
use super::TreePositions;

pub fn solve(input: &(usize, Vec<TreePositions>)) -> u64 {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result: u64 = 1;
    for slope in slopes.iter() {
        let trees_encountered = exercise_1::get_trees_encountered(&input, *slope);
        result *= trees_encountered;
    }

    result
}
