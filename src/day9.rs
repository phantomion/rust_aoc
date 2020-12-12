use std::collections::VecDeque;

pub fn day9_part1(input: &str) -> i32 {
    let mut preamble = input
        .lines()
        .take(25)
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<VecDeque<i32>>();
    let improper_number =
        input
            .lines()
            .skip(25)
            .fold(0, |improper_number, line| {
                if improper_number != 0 {
                    return improper_number
                }
                let number = line.parse::<i32>().unwrap();
                let check_number = preamble
                    .iter()
                    .find(|&&i| preamble.iter().find(|&&j| number - i == j) != None);
                match check_number {
                    Some(_) => {}
                    None => return number,
                };
                preamble.pop_front();
                preamble.push_back(number);
                improper_number
            });
    improper_number
}

pub fn day9_part2(input: &str) -> i32 {
    let improper_number = day9_part1(input);

    let (sum_vector, _) =
        input
            .lines()
            .fold((VecDeque::new(), false), |(mut sum_vector, found), line| {
                if found {
                    return (sum_vector, found);
                }
                let number = line.parse::<i32>().unwrap();
                sum_vector.push_back(number);
                loop {
                    let sum = sum_vector.iter().sum::<i32>();
                    if sum < improper_number {
                        break;
                    }
                    if sum == improper_number {
                        return (sum_vector, true);
                    }
                    sum_vector.pop_front();
                }
                (sum_vector, found)
            });
    let min = sum_vector.iter().min().unwrap();
    let max = sum_vector.iter().max().unwrap();
    min + max
}
