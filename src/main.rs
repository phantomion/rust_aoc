mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
use day1::*;
use day2::*;
use day3::*;
use day4::*;
use day5::*;
use day6::*;
use day7::*;
use day8::*;
use day9::*;
use day10::*;
use day11::*;
use day12::*;
use day13::*;
use day14::*;
use day15::*;
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
        "day9" => {
            println!("{}_part1: {}", day, day9_part1(input));
            println!("{}_part2: {}", day, day9_part2(input));
        }
        "day10" => {
            println!("{}_part1: {}", day, day10_part1(input));
            println!("{}_part2: {}", day, day10_part2(input));
        }
        "day11" | "test11" => {
            println!("{}_part1: {}", day, day11_part1(input));
            println!("{}_part2: {}", day, day11_part2(input));
        }
        "day12" => {
            println!("{}_part1: {}", day, day12_part1(input));
            println!("{}_part2: {}", day, day12_part2(input));
        }
        "day13" | "test13" => {
            println!("{}_part1: {}", day, day13_part1(input));
            println!("{}_part2: {}", day, day13_part2(input));
        }
        "day14" => {
            println!("{}_part1: {}", day, day14_part1(input));
            println!("{}_part2: {}", day, day14_part2(input));
        }
        "day15" | "test15" => {
            println!("{}_part1: {}", day, day15_part1(input));
            println!("{}_part2: {}", day, day15_part2(input));
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
