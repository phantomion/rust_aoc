mod day1;
mod day2;
mod day3;
mod day4;
use std::fs::read_to_string;
use std::env::args;
use day1::*;
use day2::*;
use day3::*;
use day4::*;

fn print_day(day: &str, input: &str) {
    match day {
       "day1" => {
           println!("{}_part1: {}", day, day1_part1(&input));
           println!("{}_part2: {}", day, day1_part2(&input));
       },
       "day2" => {
           println!("{}_part1: {}", day, day2_part1(&input));
           println!("{}_part2: {}", day, day2_part2(&input));
       },
       "day3" => {
           println!("{}_part1: {}", day, day3_part1(&input));
           println!("{}_part2: {}", day, day3_part2(&input));
       },
       "day4" => {
           println!("{}_part1: {}", day, day4_part1(&input));
           println!("{}_part2: {}", day, day4_part2(&input));
       },
       _ => println!("Day not found."),
    }

}

fn main() {
    let day = args();
    let day = day.last().unwrap();
    let input = read_to_string("input/".to_owned() + &day).unwrap_or_default();
    print_day(&day, &input);
}
