pub fn day3_part1(input: &str) -> i32 {
    let mut position = 0;
    let count = input.lines().fold(0, |acc, line| {
        let mut tree = 0;
        if position > 0 {
            let len = line.len();
            let node = line.chars().nth(position % len).unwrap();

            if node == '#' {
                tree = 1;
            }
        }
        position = position + 3;
        acc + tree
    });
    return count;
}

pub fn day3_part2(input: &str) -> i64 {
    let slope1 = get_trees_on_slope(1, 1, input);
    let slope2 = get_trees_on_slope(3, 1, input);
    let slope3 = get_trees_on_slope(5, 1, input);
    let slope4 = get_trees_on_slope(7, 1, input);
    let slope5 = get_trees_on_slope(1, 2, input);
    return slope1 * slope2 * slope3 * slope4 * slope5;
}

fn get_trees_on_slope(right: usize, down: usize, input: &str) -> i64 {
    let mut position = 0;
    let count = input.lines().step_by(down).fold(0, |acc, line| {
        let mut tree = 0;
        if position > 0 {
            let len = line.len();
            let node = line.chars().nth(position % len).unwrap();

            if node == '#' {
                tree = 1;
            }
        }
        position = position + right;
        acc + tree
    });
    return count;
}
