mod day1;
mod day2;
mod day3;
mod day4;

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
    day4::run_part_2();
}
