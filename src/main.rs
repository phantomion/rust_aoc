mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
use day1::*;
use day2::*;
use day3::*;
use day4::*;
use day5::*;
use day6::*;
use day7::*;
use day8::*;
use std::env::args;
use std::fs::read_to_string;

fn print_day(day: &str, input: &str) {
    match day {
        "day1" => {
            println!("{}_part1: {}", day, day1_part1(input));
            println!("{}_part2: {}", day, day1_part2(input));
        }
        "day2" => {
            println!("{}_part1: {}", day, day2_part1(input));
            println!("{}_part2: {}", day, day2_part2(input));
        }
        "day3" => {
            println!("{}_part1: {}", day, day3_part1(input));
            println!("{}_part2: {}", day, day3_part2(input));
        }
        "day4" => {
            println!("{}_part1: {}", day, day4_part1(input));
            println!("{}_part2: {}", day, day4_part2(input));
        }
        "day5" => {
            println!("{}_part1: {}", day, day5_part1(input));
            println!("{}_part2: {}", day, day5_part2(input));
        }
        "day6" => {
            println!("{}_part1: {}", day, day6_part1(input));
            println!("{}_part2: {}", day, day6_part2(input));
        }
        "day7" => {
            println!("{}_part1: {}", day, day7_part1(input));
            println!("{}_part2: {}", day, day7_part2(input));
        }
        "day8" => {
            println!("{}_part1: {}", day, day8_part1(input));
            println!("{}_part2: {}", day, day8_part2(input));
        }
        _ => println!("Day not found."),
    }
}

fn main() {
    let day = args();
    let day = day.last().unwrap();
    let input = read_to_string("input/".to_owned() + &day).unwrap_or_default();
    print_day(&day, &input);
}
