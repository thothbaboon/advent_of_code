use crate::read_input;
use std::fs::File;
use std::io::{self};

fn read_stones() -> Vec<usize> {
    let lines: io::Lines<io::BufReader<File>> = read_input("day11", "input.txt").unwrap();

    let line = lines
        .map_while(Result::ok)
        .last()
        .expect("Failed to read input line");

    line.split_whitespace()
        .map(|v| v.parse().expect("Should be integer"))
        .collect::<Vec<usize>>()
}

fn get_digit_count(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

pub fn run_part_1() {
    let mut stones = read_stones();
    let mut blinks = 25;

    while blinks > 0 {
        let mut next_stones = Vec::new();

        stones.into_iter().for_each(|stone| {
            if stone == 0 {
                next_stones.push(1);
            } else {
                let digits_count = get_digit_count(stone);
                if digits_count % 2 == 0 {
                    let half = digits_count / 2;
                    let divisor = 10_usize.pow(half as u32);
                    let right = stone % divisor;
                    let left = stone / divisor;
                    next_stones.push(right);
                    next_stones.push(left);
                } else {
                    next_stones.push(stone * 2024);
                }
            }
        });

        stones = next_stones;
        blinks -= 1;
    }

    println!("{}", stones.len());
}
