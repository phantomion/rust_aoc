struct Command {
    instruction: String,
    number: i32,
    used: bool,
}

pub fn day8_part1(input: &str) -> i32 {
    let mut commands = input
        .lines()
        .map(|line| {
            let command = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let sign = command[1].chars().nth(0).unwrap();
            let number = command[1]
                .strip_prefix(|c| c == '+' || c == '-')
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let number = if sign == '-' { number * (-1) } else { number };
            Command {
                instruction: command[0].to_owned(),
                number,
                used: false,
            }
        })
        .collect::<Vec<Command>>();

    let mut acc = 0;
    let mut instr = 0;
    loop {
        let (next_acc, next_instr, cycle) = execute_commands(&mut commands, acc, instr);
        if cycle {
            return acc
        }
        instr = next_instr;
        acc = next_acc;
    }
}

fn execute_commands(commands: &mut Vec<Command>, acc: i32, instr: i32) -> (i32, i32, bool) {
    let command = commands.get_mut(instr as usize).unwrap();
    if command.used {
        return (acc, instr, true)
    }
    command.used = true;
    if command.instruction == "nop" {
        return (acc, instr + 1, false)
    } else if command.instruction == "jmp" {
        return (acc, instr + command.number, false)
    } else if command.instruction == "acc" {
        return (acc + command.number, instr + 1, false)
    }
    (acc, instr, false)
}

pub fn day8_part2(input: &str) -> i32 {
    let mut commands = input
        .lines()
        .map(|line| {
            let command = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let sign = command[1].chars().nth(0).unwrap();
            let number = command[1]
                .strip_prefix(|c| c == '+' || c == '-')
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let number = if sign == '-' { number * (-1) } else { number };
            Command {
                instruction: command[0].to_owned(),
                number,
                used: false,
            }
        })
        .collect::<Vec<Command>>();

    let jmps = commands
        .iter()
        .enumerate()
        .filter(|(_, command)| command.instruction == "jmp")
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    let acc = jmps.iter().fold(0, |acc, jmp_index| {
        if acc != 0 {return acc}
        let (ex_ac, original_instr) = execute_program(&mut commands, *jmp_index, "nop");
        commands.iter_mut().for_each(|command| command.used = false);
        if let Some(command) = commands.get_mut(*jmp_index) {
            command.instruction = original_instr;
        }
        ex_ac
    });

    if acc != 0 {
        return acc
    }

    let nops = commands
        .iter()
        .enumerate()
        .filter(|(_, command)| command.instruction == "nop")
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    let acc = nops.iter().fold(0, |acc, nop_index| {
        if acc != 0 {return acc}
        let (ex_ac, original_instr) = execute_program(&mut commands, *nop_index, "jmp");
        commands.iter_mut().for_each(|command| command.used = false);
        if let Some(command) = commands.get_mut(*nop_index) {
            command.instruction = original_instr;
        }
        ex_ac
    });

    acc
}

fn execute_program(commands: &mut Vec<Command>, index: usize, instruction: &str) -> (i32, String) {
    let mut acc = 0;
    let mut instr = 0;
    let command = commands.get_mut(index).unwrap();
    let original_instr = command.instruction.clone();
    command.instruction = instruction.to_string();
    loop {
        let (next_acc, next_instr, cycle, done) = execute_commands_part2(commands, acc, instr);
        if done {
            return (acc, original_instr)
        }
        if cycle {
            return (0, original_instr)
        }
        instr = next_instr;
        acc = next_acc;
    }
}

fn execute_commands_part2(
    commands: &mut Vec<Command>,
    acc: i32,
    instr: i32,
) -> (i32, i32, bool, bool) {
    let command = commands.get_mut(instr as usize);
    let command = match command {
        Some(command) => command,
        None => return (acc, instr, false, true),
    };
    if command.used {
        return (acc, instr, true, false)
    }
    command.used = true;
    if command.instruction == "nop" {
        return (acc, instr + 1, false, false)
    } else if command.instruction == "jmp" {
        return (acc, instr + command.number, false, false)
    } else if command.instruction == "acc" {
        return (acc + command.number, instr + 1, false, false)
    }
    (acc, instr, false, false)
}
