pub fn day10_part1(input: &str) -> i32 {
    let mut adapters = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    adapters.sort();
    let (jolt_diff1, jolt_diff3) = adapters
        .windows(2)
        .collect::<Vec<_>>()
        .iter()
        .fold((0, 0), |(jolt_diff1, jolt_diff3), &jolt| {
        if jolt[1] - jolt[0] == 1 {
            return (jolt_diff1 + 1, jolt_diff3)
        }
        else if jolt[1] - jolt[0] == 3 {
            return (jolt_diff1, jolt_diff3 + 1)
        }
        (jolt_diff1, jolt_diff3)
    });
    jolt_diff1 * (jolt_diff3 + 1)
}

pub fn day10_part2(input: &str) -> i64 {
    let mut adapters = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    adapters.push(0);
    adapters.sort();
    adapters
        .windows(2)
        .collect::<Vec<_>>()
        .split(|n| n[1] - n[0] == 3)
        .map(|n| match n.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
    .product::<i64>()
}
