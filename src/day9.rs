pub fn day9_part1(input: &str) -> i32 {
    let mut preamble = input
        .lines()
        .take(25)
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let (improper_number, _) =
        input
            .lines()
            .skip(25)
            .fold((0, 0), |(improper_number, last_preamble), line| {
                if improper_number != 0 {
                    return (improper_number, last_preamble)
                }
                let number = line.parse::<i32>().unwrap();
                let check_number = preamble
                    .iter()
                    .find(|&&i| preamble.iter().find(|&&j| number - i == j) != None);
                match check_number {
                    Some(_) => {}
                    None => return (number, last_preamble),
                };
                preamble.insert(last_preamble % 25, number);
                (improper_number, last_preamble + 1)
            });
    improper_number
}

pub fn day9_part2(input: &str) -> i32 {
    let mut preamble = input
        .lines()
        .take(25)
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let (improper_number, _) =
        input
            .lines()
            .skip(25)
            .fold((0, 0), |(improper_number, last_preamble), line| {
                if improper_number != 0 {
                    return (improper_number, last_preamble)
                }
                let number = line.parse::<i32>().unwrap();
                let check_number = preamble
                    .iter()
                    .find(|&&i| preamble.iter().find(|&&j| number - i == j) != None);
                match check_number {
                    Some(_) => {}
                    None => return (number, last_preamble),
                };
                preamble.insert(last_preamble % 25, number);
                (improper_number, last_preamble + 1)
            });

    let (sum_vector, _) = input.lines().fold((Vec::new(), false), |(mut sum_vector, found), line| {
        if found {return (sum_vector, found)}
        let number = line.parse::<i32>().unwrap();
        sum_vector.push(number);
        loop {
            let sum = sum_vector.iter().sum::<i32>();
            if sum < improper_number {
                break;
            }
            if sum == improper_number {
                return (sum_vector, true)
            }
            sum_vector.remove(0);
        }
        (sum_vector, found)
    });
    let min = sum_vector.iter().min().unwrap();
    let max = sum_vector.iter().max().unwrap();
    min + max
}
