use super::TreePositions;

const SLOPE: (usize, usize) = (3, 1);

pub fn solve(input: &(usize, Vec<TreePositions>)) -> u64 {
    get_trees_encountered(input, SLOPE)
}

pub fn get_trees_encountered(input: &(usize, Vec<TreePositions>), slope: (usize, usize)) -> u64 {
    let (pattern_width, tree_positions_per_row) = input;

    let mut trees_encountered = 0;
    let mut slope_coordinates = slope;
    for (index, tree_positions) in tree_positions_per_row.iter().enumerate() {
        if slope_coordinates.1 != index {
            continue;
        }

        if let Some(_) = tree_positions.iter().find(|&&x| x == slope_coordinates.0) {
            trees_encountered += 1;
        }

        // Increment y coordinate
        slope_coordinates.1 += slope.1;

        // Increment x coordinate and loop
        slope_coordinates.0 += slope.0;
        slope_coordinates.0 %= pattern_width;
    }

    trees_encountered
}

#[cfg(test)]
mod tests {
    /// Input represents:
    ///
    ///     ......
    ///     ...#..
    ///     #.....
    ///     ...#..
    ///
    ///
    fn get_input() -> (usize, Vec<super::TreePositions>) {
        (6, vec![vec![], vec![3], vec![0], vec![3]])
    }

    #[test]
    fn solution_works() {
        let input = get_input();

        let trees_encountered = super::solve(input);
        assert_eq!(trees_encountered, 3);
    }
}
