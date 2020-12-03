pub fn day2_part1(input: &str) -> i32 {
    let count = input.lines().fold(0, |acc, line| {
        let (requirements, password) = {
            let line = line.split(':').collect::<Vec<&str>>();
            (line[0], line[1])
        };
        let (numbers, letter) = {
            let requirements = requirements.split_ascii_whitespace().collect::<Vec<&str>>();
            (requirements[0], requirements[1])
        };
        let (min, max) = {
            let numbers = numbers.split('-').collect::<Vec<&str>>();
            (numbers[0].parse().unwrap(), numbers[1].parse().unwrap())
        };
        let count = password
            .chars()
            .into_iter()
            .filter(|character| *character == letter.chars().nth(0).unwrap())
            .count();
        if count >= min && count <= max {
            return acc + 1;
        } else {
            return acc;
        }
    });
    return count;
}

pub fn day2_part2(input: &str) -> i32 {
    let count = input.lines().fold(0, |acc, line| {
        let (requirements, password) = {
            let line = line.split(':').collect::<Vec<&str>>();
            (line[0], line[1].trim())
        };
        let (numbers, letter) = {
            let requirements = requirements.split_ascii_whitespace().collect::<Vec<&str>>();
            (requirements[0], requirements[1])
        };
        let (first, second): (usize, usize) = {
            let numbers = numbers.split('-').collect::<Vec<&str>>();
            (numbers[0].parse().unwrap(), numbers[1].parse().unwrap())
        };
        let letter = letter.chars().nth(0).unwrap();
        let (pass_first, pass_second) = {
            (password.chars().nth(first - 1).unwrap(), password.chars().nth(second - 1).unwrap())
        };
        if pass_first == letter && pass_second != letter {
            return acc + 1;
        }
        else if pass_first != letter && pass_second == letter {
            return acc + 1;
        }
        else {
            return acc;
        }
    });
    return count;
}
