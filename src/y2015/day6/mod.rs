use std::collections::{HashMap, HashSet};

use crate::read_input;

enum InstructionKind {
    Toggle,
    TurnOff,
    TurnOn,
}

impl From<&str> for InstructionKind {
    fn from(s: &str) -> InstructionKind {
        match s {
            "turn off" => InstructionKind::TurnOff,
            "turn on" => InstructionKind::TurnOn,
            "toggle" => InstructionKind::Toggle,
            _ => panic!("Unexpected instruction kind {s}"),
        }
    }
}

struct Coordinate {
    x: usize,
    y: usize,
}

impl From<&str> for Coordinate {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

struct Instruction {
    kind: InstructionKind,
    bottom_left: Coordinate,
    top_right: Coordinate,
}

fn read_instructions() -> Vec<Instruction> {
    read_input(2015, 6)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (first_part, top_right_str) = line.split_once(" through ").unwrap();
            let (instruction_kind_str, bottom_left_str) = first_part.rsplit_once(" ").unwrap();

            Instruction {
                kind: instruction_kind_str.into(),
                bottom_left: bottom_left_str.into(),
                top_right: top_right_str.into(),
            }
        })
        .collect()
}

pub fn run_part_1() {
    let instructions = read_instructions();
    let mut lights_on = HashSet::new();

    for instruction in instructions {
        for i in instruction.bottom_left.x..=instruction.top_right.x {
            for j in instruction.bottom_left.y..=instruction.top_right.y {
                match instruction.kind {
                    InstructionKind::TurnOn => lights_on.insert((i, j)),
                    InstructionKind::TurnOff => lights_on.remove(&(i, j)),
                    InstructionKind::Toggle => {
                        if lights_on.contains(&(i, j)) {
                            lights_on.remove(&(i, j))
                        } else {
                            lights_on.insert((i, j))
                        }
                    }
                };
            }
        }
    }

    assert_eq!(lights_on.len(), 400410);
}

pub fn run_part_2() {
    let instructions = read_instructions();
    let mut lights_brightness = HashMap::new();

    for instruction in instructions {
        for i in instruction.bottom_left.x..=instruction.top_right.x {
            for j in instruction.bottom_left.y..=instruction.top_right.y {
                match instruction.kind {
                    InstructionKind::TurnOn => *lights_brightness.entry((i, j)).or_default() += 1,
                    InstructionKind::TurnOff => {
                        let b = lights_brightness.entry((i, j)).or_default();
                        *b = (0).max(&*b - 1);
                    }
                    InstructionKind::Toggle => *lights_brightness.entry((i, j)).or_default() += 2,
                };
            }
        }
    }

    let total: isize = lights_brightness.values().sum();
    assert_eq!(total, 15343601);
}
