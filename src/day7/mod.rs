use crate::read_input;
use std::collections::HashSet;

struct CalibrationEquation {
    test_value: usize,
    numbers: Vec<usize>,
}

fn parse_line(line: &str) -> CalibrationEquation {
    let coma_sides: Vec<&str> = line.split(": ").collect();

    CalibrationEquation {
        test_value: coma_sides[0].parse::<usize>().expect("Result is not usize"),
        numbers: coma_sides[1]
            .split(" ")
            .map(|c| c.parse::<usize>().expect("Component is not usize"))
            .collect(),
    }
}

fn concat_numbers(a: usize, b: usize) -> usize {
    let mut b_copy = b;
    let mut factor = 1;

    while b_copy > 0 {
        b_copy /= 10;
        factor *= 10;
    }

    a * factor + b
}

impl CalibrationEquation {
    fn is_valid(&mut self, with_concat: bool) -> bool {
        let mut intermediate_values: HashSet<usize> = HashSet::new();
        intermediate_values.insert(self.numbers[0]);

        for i in 1..self.numbers.len() {
            let current: Vec<_> = intermediate_values.drain().collect();

            for v in current {
                let a = self.numbers[i] + v;
                if a <= self.test_value {
                    intermediate_values.insert(a);
                }

                let m = self.numbers[i] * v;
                if m <= self.test_value {
                    intermediate_values.insert(m);
                }

                // operator for part 2
                if with_concat {
                    let c = concat_numbers(v, self.numbers[i]);
                    if c <= self.test_value {
                        intermediate_values.insert(c);
                    }
                }
            }
        }

        intermediate_values.contains(&self.test_value)
    }
}

pub fn run_part_1() {
    let lines = read_input("day7", "input.txt").unwrap();

    let total_calibration_result = lines
        .map_while(Result::ok)
        .map(|line| parse_line(&line))
        .fold(0, |total_calibration_result, mut equation| {
            if equation.is_valid(false) {
                total_calibration_result + equation.test_value
            } else {
                total_calibration_result
            }
        });

    println!("{}", total_calibration_result);
}

pub fn run_part_2() {
    let lines = read_input("day7", "input.txt").unwrap();

    let total_calibration_result = lines
        .map_while(Result::ok)
        .map(|line| parse_line(&line))
        .fold(0, |total_calibration_result, mut equation| {
            if equation.is_valid(true) {
                total_calibration_result + equation.test_value
            } else {
                total_calibration_result
            }
        });

    println!("{}", total_calibration_result);
}
