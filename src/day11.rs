pub fn day11_part1(input: &str) -> usize {
    let mut seat_line = input
        .lines()
        .map(|line| line.chars().map(|c| (c, c)).collect::<Vec<(char, char)>>())
        .collect::<Vec<_>>();

    loop {
        seat_line = seat_line
            .iter()
            .enumerate()
            .map(|(index, current_line)| {
                current_line
                    .iter()
                    .enumerate()
                    .map(|(char_index, (current, next))| {
                        if *current == '.' {
                            return (*current, *next);
                        }
                        let occupied_seats = {
                            let occupied_seats1 =
                                get_line(index as i32 - 1, char_index, &seat_line);
                            let occupied_seats2 = get_line(index as i32, char_index, &seat_line);
                            let occupied_seats3 =
                                get_line(index as i32 + 1, char_index, &seat_line);
                            if *current == '#' {
                                occupied_seats1 + occupied_seats2 + occupied_seats3 - 1
                            } else {
                                occupied_seats1 + occupied_seats2 + occupied_seats3
                            }
                        };

                        if occupied_seats >= 4 && *current == '#' {
                            return (*current, 'L');
                        } else if occupied_seats == 0 && *current == 'L' {
                            return (*current, '#');
                        }
                        (*current, *current)
                    })
                    .collect::<Vec<(char, char)>>()
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|current_line| {
                current_line
                    .iter()
                    .map(|(current, next)| (*next, *current))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let done = seat_line.iter().fold(true, |done, line| {
            if !done {
                return done
            }
            line.iter().fold(true, |done, (current, next)| {
                if !done {
                    return done
                }
                *current == *next
            })
        });

        if done {
            return seat_line.iter().fold(0, |occupied, line| {
                occupied + line.iter().filter(|(current, _)| *current == '#').count()
            });
        }
    }
}

fn get_line(index: i32, char_index: usize, seat_line: &Vec<Vec<(char, char)>>) -> i32 {
    if let Some(line) = seat_line.get(index as usize) {
        let occupied_seat1 = get_seats_in_line(char_index as i32 - 1, &line);
        let occupied_seat2 = get_seats_in_line(char_index as i32, &line);
        let occupied_seat3 = get_seats_in_line(char_index as i32 + 1, &line);
        return occupied_seat1 + occupied_seat2 + occupied_seat3;
    }
    0
}

fn get_seats_in_line(char_index: i32, line: &Vec<(char, char)>) -> i32 {
    if let Some(c) = line.get(char_index as usize) {
        if c.0 == '#' {
            return 1;
        }
    }
    0
}

pub fn day11_part2(input: &str) -> i32 {
    1
}
