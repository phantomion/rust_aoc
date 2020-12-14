use std::collections::HashMap;

pub fn day14_part1(input: &str) -> isize {
    let mut addr = HashMap::new();
    let _ = input.lines().fold("", |mask, line| {
        let contents = line.split(" = ").collect::<Vec<&str>>();
        match contents[0] {
            "mask" => return contents[1],
            _ => {
                let address = contents[0]
                    .chars()
                    .filter(|c| c.is_numeric())
                    .map(|num| num)
                    .collect::<String>();
                let address = address.parse::<usize>().unwrap();
                let bin_val = format!("{:036b}", contents[1].parse::<i32>().unwrap());
                let result = mask
                    .chars()
                    .enumerate()
                    .map(|(index, bit)| {
                        return if bit == 'X' {
                            bin_val.chars().nth(index).unwrap()
                        } else {
                            bit
                        };
                    })
                    .collect::<String>();
                let result = isize::from_str_radix(result.as_str(), 2).unwrap();
                addr.insert(address, result);
            }
        };
        mask
    });

    addr.values().sum()
}

pub fn day14_part2(input: &str) -> isize {
    let mut addr = HashMap::new();
    let _ = input.lines().fold("", |mask, line| {
        let contents = line.split(" = ").collect::<Vec<&str>>();
        match contents[0] {
            "mask" => return contents[1],
            _ => {
                let address = contents[0]
                    .chars()
                    .filter(|c| c.is_numeric())
                    .map(|num| num)
                    .collect::<String>();
                let address = address.parse::<usize>().unwrap();
                let bin_addr = format!("{:036b}", address);
                let value = contents[1].parse::<isize>().unwrap();
                let result = mask
                    .chars()
                    .enumerate()
                    .map(|(index, bit)| {
                        return if bit == '0' {
                            bin_addr.chars().nth(index).unwrap()
                        } else {
                            bit
                        };
                    })
                    .collect::<String>();
                let floating_count = result.chars().enumerate().filter(|(_, x)| *x == 'X').fold(
                    Vec::new(),
                    |mut floatings, (i, _)| {
                        floatings.push(i);
                        floatings
                    },
                );
                let combos = 2i32.pow(floating_count.len() as u32);
                for i in 0..combos {
                    let bit_combo = format!("{:b}", i);
                    let mut bit_index = bit_combo.len() as i32;
                    let result = result
                        .chars()
                        .rev()
                        .map(|bit| {
                            return if bit == 'X' {
                                if bit_index > 0 {
                                    bit_index -= 1;
                                    bit_combo.chars().nth(bit_index as usize).unwrap()
                                } else {
                                    '0'
                                }
                            } else {
                                bit
                            };
                        })
                        .collect::<String>()
                        .chars()
                        .rev()
                        .map(|bit| bit)
                        .collect::<String>();
                    let address = isize::from_str_radix(result.as_str(), 2).unwrap();
                    addr.insert(address, value);
                }
            }
        };
        mask
    });

    addr.values().sum()
}
