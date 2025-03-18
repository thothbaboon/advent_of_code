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

use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input(day: &str, path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(format!("src/{}/{}", day, path))?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // day1::run_part_1();
    // day1::run_part_2();
    // day2::run_part_1();
    // day2::run_part_2();
    // day3::run_part_1();
    // day3::run_part_2();
    // day4::run_part_1();
    // day4::run_part_2();
    // day5::run_part_1();
    // day5::run_part_2();
    // day6::run_part_1();
    // day6::run_part_2();
    // day7::run_part_1();
    // day7::run_part_2();
    // day8::run_part_1();
    // day8::run_part_2();
    // day9::run_part_1();
    // day9::run_part_2();
    day10::run_part_1();
}
