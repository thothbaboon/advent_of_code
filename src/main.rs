mod y2024;
mod y2020;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input(year: u16, day: u8) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(format!("src/y{}/day{}/input.txt", year, day))?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: cargo run -- [year] [day] [part]");
        eprintln!("Example: cargo run -- 2024 1 2");
        return;
    }

    let year = &args[1];
    let day = &args[2];
    let part = &args[3];

    match (year.as_str(), day.as_str(), part.as_str()) {
        ("2020", "1", "1") => y2020::day1::run_part_1(),
        ("2020", "1", "2") => y2020::day1::run_part_2(),
        ("2020", "2", "1") => y2020::day2::run_part_1(),
        ("2020", "2", "2") => y2020::day2::run_part_2(),
        ("2020", "3", "1") => y2020::day3::run_part_1(),
        ("2020", "3", "2") => y2020::day3::run_part_2(),

        ("2024", "1", "1") => y2024::day1::run_part_1(),
        ("2024", "1", "2") => y2024::day1::run_part_2(),
        ("2024", "2", "1") => y2024::day2::run_part_1(),
        ("2024", "2", "2") => y2024::day2::run_part_2(),
        ("2024", "3", "1") => y2024::day3::run_part_1(),
        ("2024", "3", "2") => y2024::day3::run_part_2(),
        ("2024", "4", "1") => y2024::day4::run_part_1(),
        ("2024", "4", "2") => y2024::day4::run_part_2(),
        ("2024", "5", "1") => y2024::day5::run_part_1(),
        ("2024", "5", "2") => y2024::day5::run_part_2(),
        ("2024", "6", "1") => y2024::day6::run_part_1(),
        ("2024", "6", "2") => y2024::day6::run_part_2(),
        ("2024", "7", "1") => y2024::day7::run_part_1(),
        ("2024", "7", "2") => y2024::day7::run_part_2(),
        ("2024", "8", "1") => y2024::day8::run_part_1(),
        ("2024", "8", "2") => y2024::day8::run_part_2(),
        ("2024", "9", "1") => y2024::day9::run_part_1(),
        ("2024", "9", "2") => y2024::day9::run_part_2(),
        ("2024", "10", "1") => y2024::day10::run_part_1(),
        ("2024", "10", "2") => y2024::day10::run_part_2(),
        ("2024", "11", "1") => y2024::day11::run_part_1(),
        ("2024", "11", "2") => y2024::day11::run_part_2(),
        ("2024", "12", "1") => y2024::day12::run_part_1(),
        ("2024", "12", "2") => y2024::day12::run_part_2(),
        ("2024", "13", "1") => y2024::day13::run_part_1(),
        ("2024", "13", "2") => y2024::day13::run_part_2(),
        ("2024", "14", "1") => y2024::day14::run_part_1(),
        ("2024", "14", "2") => y2024::day14::run_part_2(),
        ("2024", "15", "1") => y2024::day15::run_part_1(),
        ("2024", "15", "2") => y2024::day15::run_part_2(),
        ("2024", "16", "1") => y2024::day16::run_part_1(),
        ("2024", "16", "2") => y2024::day16::run_part_2(),
        ("2024", "17", "1") => y2024::day17::run_part_1(),
        ("2024", "17", "2") => y2024::day17::run_part_2(),
        ("2024", "18", "1") => y2024::day18::run_part_1(),
        ("2024", "18", "2") => y2024::day18::run_part_2(),
        ("2024", "19", "1") => y2024::day19::run_part_1(),
        ("2024", "19", "2") => y2024::day19::run_part_2(),
        ("2024", "20", "1") => y2024::day20::run_part_1(),
        ("2024", "20", "2") => y2024::day20::run_part_2(),
        ("2024", "21", "1") => y2024::day21::run_part_1(),
        ("2024", "21", "2") => y2024::day21::run_part_2(),
        ("2024", "22", "1") => y2024::day22::run_part_1(),
        ("2024", "22", "2") => y2024::day22::run_part_2(),
        ("2024", "23", "1") => y2024::day23::run_part_1(),
        ("2024", "23", "2") => y2024::day23::run_part_2(),
        ("2024", "24", "1") => y2024::day24::run_part_1(),
        ("2024", "24", "2") => y2024::day24::run_part_2(),
        ("2024", "25", "1") => y2024::day25::run_part_1(),
        _ => eprintln!("Invalid year, day, or part"),
    }
}
