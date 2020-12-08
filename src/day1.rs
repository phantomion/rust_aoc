pub fn day1_part1(input: &str) -> i32 {
    let mut hash_table: [i32; 2020] = [0; 2020];
    let input = input
        .lines()
        .map(|number| {
            let index = number.parse::<usize>().unwrap();
            hash_table[index] = 1;
            number.parse().unwrap()
        })
        .collect::<Vec<i32>>();

    let result = input
        .iter()
        .find(|&&number| {
            let index = number as usize;
            hash_table[2020 - index] == 1
        })
        .unwrap()
        .to_owned();

    result * (2020 - result)
}

pub fn day1_part2(input: &str) -> i32 {
    let input = input
        .lines()
        .map(|number| number.parse().unwrap())
        .collect::<Vec<i32>>();

    let total = input.iter().fold(0, |total, number| {
        if total != 0 {
            return total;
        }
        input.iter().fold(0, |total, number2| {
            if total != 0 {
                return total;
            }
            input.iter().fold(0, |total, number3| {
                if number + number2 + number3 == 2020 {
                    return number * number2 * number3;
                }
                total
            })
        })
    });

    total
}
