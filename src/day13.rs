pub fn day13_part1(input: &str) -> i32 {
    let split_input = input.split_ascii_whitespace().collect::<Vec<&str>>();
    let timestamp = split_input[0].parse::<i32>().unwrap();
    let buses = split_input[1].split(",").collect::<Vec<&str>>();
    let (min_wait_time, bus_id) = buses
        .iter()
        .filter(|bus| **bus != "x")
        .fold((timestamp, 0), |(min_wait_time, min_bus_id), bus| {
            let bus_id = bus.parse::<i32>().unwrap();
            return if min_wait_time > bus_id - (timestamp % bus_id) {
                (bus_id - (timestamp % bus_id), bus_id)
            }
            else {
                (min_wait_time, min_bus_id)
            }
        });
    min_wait_time * bus_id
}

pub fn day13_part2(_input: &str) -> i32 {
    1
}
