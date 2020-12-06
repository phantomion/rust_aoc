use std::cmp::max;

pub fn day5_part1(input: &str) -> i32 {
    let highest_id = input.lines().fold(0, |id, line| {
        let row = line
            .chars()
            .take(7)
            .fold((0, 127), |(min, max), direction| {
                let (min, max) = match direction {
                    'B' => ((max + min) / 2 + 1, max),
                    'F' => (min, (max + min) / 2),
                    _ => (0, 0),
                };
                (min, max)
            });

        let col = line.chars().skip(7).fold((0, 7), |(min, max), direction| {
            let (min, max) = match direction {
                'R' => ((max + min) / 2 + 1, max),
                'L' => (min, (max + min) / 2),
                _ => (0, 0),
            };
            (min, max)
        });
        max(id, row.0 * 8 + col.0)
    });

    return highest_id;
}

pub fn day5_part2(input: &str) -> i32 {
    let seat_ids = input.lines().fold(Vec::new(), |mut ids, line| {
        let row = line
            .chars()
            .take(7)
            .fold((0, 127), |(min, max), direction| {
                let (min, max) = match direction {
                    'B' => ((max + min) / 2 + 1, max),
                    'F' => (min, (max + min) / 2),
                    _ => (0, 0),
                };
                (min, max)
            });

        let col = line.chars().skip(7).fold((0, 7), |(min, max), direction| {
            let (min, max) = match direction {
                'R' => ((max + min) / 2 + 1, max),
                'L' => (min, (max + min) / 2),
                _ => (0, 0),
            };
            (min, max)
        });
        ids.push(row.0 * 8 + col.0);
        ids
    });

    let seat_id = (0..1024)
        .skip(8)
        .take_while(|&id| id != 127 * 8)
        .find(|id| {
            !seat_ids.contains(&id) && seat_ids.contains(&(id + 1)) && seat_ids.contains(&(id - 1))
        })
        .unwrap();
    return seat_id;
}
