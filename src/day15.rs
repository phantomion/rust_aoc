use std::collections::HashMap;

pub fn day15_part1(input: &str) -> usize {
    let starting_numbers = input
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|num| num.trim_end_matches('\n').parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (_, last_number) =
        (0..2020).fold((HashMap::new(), 0), |(mut numbers, last_spoken), turn| {
            return if turn < starting_numbers.len() {
                let current_num = starting_numbers.get(turn).unwrap();
                numbers.insert(*current_num, (turn, turn));
                (numbers, *current_num)
            } else {
                if let Some(number) = numbers.get(&last_spoken) {
                    let mrs = number.0;
                    let last_turn = number.1;
                    if mrs == last_turn {
                        if let Some(turns) = numbers.get_mut(&0) {
                            turns.0 = turns.1;
                            turns.1 = turn;
                        } else {
                            numbers.insert(0, (turn, turn));
                        }
                        return (numbers, 0);
                    }
                    let next_number = last_turn - mrs;
                    if let Some(next) = numbers.get_mut(&next_number) {
                        next.0 = next.1;
                        next.1 = turn;
                    } else {
                        numbers.insert(next_number, (turn, turn));
                    }
                    return (numbers, next_number);
                }
                (numbers, last_spoken)
            };
        });
    last_number
}

pub fn day15_part2(input: &str) -> usize {
    let starting_numbers = input
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|num| num.trim_end_matches('\n').parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (_, last_number) =
        (0..30000000).fold((HashMap::new(), 0), |(mut numbers, last_spoken), turn| {
            return if turn < starting_numbers.len() {
                let current_num = starting_numbers.get(turn).unwrap();
                numbers.insert(*current_num, (turn, turn));
                (numbers, *current_num)
            } else {
                if let Some(number) = numbers.get(&last_spoken) {
                    let mrs = number.0;
                    let last_turn = number.1;
                    if mrs == last_turn {
                        if let Some(turns) = numbers.get_mut(&0) {
                            turns.0 = turns.1;
                            turns.1 = turn;
                        } else {
                            numbers.insert(0, (turn, turn));
                        }
                        return (numbers, 0);
                    }
                    let next_number = last_turn - mrs;
                    if let Some(next) = numbers.get_mut(&next_number) {
                        next.0 = next.1;
                        next.1 = turn;
                    } else {
                        numbers.insert(next_number, (turn, turn));
                    }
                    return (numbers, next_number);
                }
                (numbers, last_spoken)
            };
        });
    last_number
}
