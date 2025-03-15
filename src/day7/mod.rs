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

impl CalibrationEquation {
    fn is_valid(&mut self) -> bool {
        let mut intermediate_values: HashSet<usize> = HashSet::new();
        intermediate_values.insert(self.numbers[0]);

        self.numbers[1..].iter().for_each(|component| {
            let current: Vec<_> = intermediate_values.drain().collect();
            for v in current {
                let a = component + v;
                if a <= self.test_value {
                    intermediate_values.insert(a);
                }

                let m = component * v;
                if m <= self.test_value {
                    intermediate_values.insert(m);
                }
            }
        });

        intermediate_values.contains(&self.test_value)
    }
}

pub fn run_part_1() {
    let lines = read_input("day7", "input.txt").unwrap();

    let total_calibration_result = lines
        .map_while(Result::ok)
        .map(|line| parse_line(&line))
        .fold(0, |total_calibration_result, mut equation| {
            if equation.is_valid() {
                total_calibration_result + equation.test_value
            } else {
                total_calibration_result
            }
        });

    println!("{}", total_calibration_result);
}
