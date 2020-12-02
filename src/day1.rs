pub fn day1_part1(input: &str) -> i32 {
    let mut hash_table: [i32; 2020] = [0; 2020];
    let input = input
        .lines()
        .map(|number| {
            let index = number.parse::<usize>().unwrap();
            hash_table[index] = 1;
            return number.parse().unwrap();
        })
        .collect::<Vec<i32>>();

    let result = input.iter().find(|&&number| {
        let index = number as usize;
        hash_table[2020 - index] == 1
    });

    let result = *result.unwrap() * (2020 - *result.unwrap());
    return result;
}

pub fn day1_part2(input: &str) -> i32 {
    let mut hash_table: [i32; 2019] = [0; 2019];
    let mut _hash_table2: [i32; 2019] = [0; 2019];

    let _input = input
        .lines()
        .map(|number| {
            let index = number.parse::<usize>().unwrap();
            hash_table[index] = 1;
            number.parse().unwrap()
        })
        .collect::<Vec<i32>>();

    let _result = hash_table
        .iter()
        .filter(|&&number| number == 1)
        .map(|_number| {
            //iterate input here
        });
    return 1;
}
