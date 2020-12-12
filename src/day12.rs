pub fn day12_part1(input: &str) -> i32 {
    let (x, y, _) = input.lines().fold((0, 0, 'E'), |(x, y, facing), line| {
        let direction = line.chars().nth(0).unwrap();
        let units = line
            .strip_prefix(direction)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let facing_number = match facing {
            'E' => 0,
            'S' => 1,
            'W' => 2,
            'N' => 3,
            _ => unreachable!(),
        };
        if direction == 'F' {
            return get_coords(x, y, units, facing, facing);
        } else if direction == 'R' {
            let facing_number = facing_number + (units / 90);
            let direction = left_right_direction(facing_number);
            return get_coords(x, y, 0, direction, direction);
        } else if direction == 'L' {
            let facing_number = facing_number - (units / 90);
            let direction = left_right_direction(facing_number);
            return get_coords(x, y, 0, direction, direction);
        } else {
            return get_coords(x, y, units, direction, facing);
        }
    });
    x.abs() + y.abs()
}

fn left_right_direction(facing_number: i32) -> char {
    return if facing_number.rem_euclid(4) == 3 {
        'N'
    } else if facing_number.rem_euclid(4) == 2 {
        'W'
    } else if facing_number.rem_euclid(4) == 1 {
        'S'
    } else {
        'E'
    };
}

fn get_coords(x: i32, y: i32, units: i32, direction: char, facing: char) -> (i32, i32, char) {
    match direction {
        'E' => return (x + units, y, facing),
        'W' => return (x - units, y, facing),
        'N' => return (x, y + units, facing),
        'S' => return (x, y - units, facing),
        _ => unreachable!(),
    }
}

pub fn day12_part2(input: &str) -> i32 {
    let (x, y, _, _) = input.lines().fold((0, 0, 10, 1), |(x, y, xw, yw), line| {
        let direction = line.chars().nth(0).unwrap();
        let units = line
            .strip_prefix(direction)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let facing_x = if xw >= 0 { 'E' } else { 'W' };
        let facing_y = if yw >= 0 { 'N' } else { 'S' };
        let facing_number_x = match facing_x {
            'E' => 0,
            'S' => 1,
            'W' => 2,
            'N' => 3,
            _ => unreachable!(),
        };
        let facing_number_y = match facing_y {
            'E' => 0,
            'S' => 1,
            'W' => 2,
            'N' => 3,
            _ => unreachable!(),
        };
        let (facing_number_x, facing_number_y) = if direction == 'R' {
            (facing_number_x + (units / 90),
            facing_number_y + (units / 90))
        }
        else if direction == 'L' {
            (facing_number_x - (units / 90),
            facing_number_y - (units / 90))
        }
        else {
            (facing_number_x, facing_number_y)
        };
        if direction == 'R' || direction == 'L' {
            let direction_x = left_right_direction(facing_number_x);
            let direction_y = left_right_direction(facing_number_y);
            let (xw, yw) = get_facing_waypoint(xw, yw, direction_x, direction_y);
            return get_coords_part2(x, y, xw, yw, units, direction)
        }
        get_coords_part2(x, y, xw, yw, units, direction)
    });
    x.abs() + y.abs()
}

fn get_facing_waypoint(xw: i32, yw: i32, direction_x: char, direction_y: char) -> (i32, i32) {
    let (xw, yw, direction_x, direction_y) = if direction_x == 'N' || direction_x == 'S' {
        (yw, xw, direction_y, direction_x)
    } else {
        (xw, yw, direction_x, direction_y)
    };
    let xw = if (direction_x == 'E' && xw < 0) || (direction_x == 'W' && xw > 0) {
        -xw
    } else {
        xw
    };
    let yw = if (direction_y == 'N' && yw < 0) || (direction_y == 'S' && yw > 0) {
        -yw
    } else {
        yw
    };
    (xw, yw)
}

fn get_coords_part2(
    x: i32,
    y: i32,
    xw: i32,
    yw: i32,
    units: i32,
    direction: char,
) -> (i32, i32, i32, i32) {
    match direction {
        'E' => return (x, y, xw + units, yw),
        'W' => return (x, y, xw - units, yw),
        'N' => return (x, y, xw, yw + units),
        'S' => return (x, y, xw, yw - units),
        'F' => return (x + xw * units, y + yw * units, xw, yw),
        'R' | 'L' => return (x, y, xw, yw),
        _ => unreachable!(),
    }
}
