mod day1;
use std::fs::read_to_string;
use std::env::args;
use day1::*;

fn print_day(day: &str, input: &str) {
    match day {
       "day1" => {
           println!("{}_part1: {}", day, day1_part1(&input));
           println!("{}_part2: {}", day, day1_part2(&input));
       },
       _ => println!("Day not found."),
    }

}

fn main() {
    let day = args();
    let day = day.last().unwrap();
    let input = read_to_string("input/".to_owned() + &day).unwrap();
    print_day(&day, &input);
}
