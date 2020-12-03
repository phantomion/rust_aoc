pub fn day2_part1(input: &str) -> i32 {
    let count = input.lines().fold(0, |acc, line| {
        let line = line.split(':').collect::<Vec<&str>>();
        let (requirements, password) = (line[0], line[1]);
        let requirements = requirements.split_ascii_whitespace().collect::<Vec<&str>>();
        let (numbers, letter) = (requirements[0], requirements[1]);
        let numbers = numbers.split('-').collect::<Vec<&str>>();
        let (min, max): (usize, usize) = (numbers[0].parse().unwrap(), numbers[1].parse().unwrap());
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
        let line = line.split(':').collect::<Vec<&str>>();
        let (requirements, password) = (line[0], line[1]);
        let requirements = requirements.split_ascii_whitespace().collect::<Vec<&str>>();
        let (numbers, letter) = (requirements[0], requirements[1]);
        let numbers = numbers.split('-').collect::<Vec<&str>>();
        let (min, max): (usize, usize) = (numbers[0].parse().unwrap(), numbers[1].parse().unwrap());
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
