use std::collections::{HashMap, VecDeque};

use crate::read_input;
mod dot;

type Wire = String;

#[derive(Clone, Debug)]
enum Operation {
    WireValue(Wire),
    ValueInt(usize),
    Not(Wire),
    And(Wire, Wire),
    Or(Wire, Wire),
    Lshift(Wire, usize),
    Rshift(Wire, usize),
}

#[derive(Debug)]
struct Instruction {
    target_wire: String,
    operation: Operation,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (left, target_wire_str) = value.split_once(" -> ").unwrap();
        let target_wire: Wire = target_wire_str.into();

        if left.starts_with("NOT") {
            let (_, source_wire_str) = left.split_once(" ").unwrap();
            return Instruction {
                target_wire,
                operation: Operation::Not(source_wire_str.into()),
            };
        }

        if !left.contains(" ") {
            if let Ok(value) = left.parse::<usize>() {
                return Instruction {
                    target_wire,
                    operation: Operation::ValueInt(value),
                };
            } else {
                return Instruction {
                    target_wire,
                    operation: Operation::WireValue(left.to_string()),
                };
            }
        }

        let parts: Vec<&str> = left.split_whitespace().collect();

        match parts[1] {
            "AND" => Instruction {
                target_wire,
                operation: Operation::And(parts[0].into(), parts[2].into()),
            },
            "OR" => Instruction {
                target_wire,
                operation: Operation::Or(parts[0].into(), parts[2].into()),
            },
            "LSHIFT" => Instruction {
                target_wire,
                operation: Operation::Lshift(parts[0].into(), parts[2].parse().unwrap()),
            },
            "RSHIFT" => Instruction {
                target_wire,
                operation: Operation::Rshift(parts[0].into(), parts[2].parse().unwrap()),
            },
            _ => panic!("Unexpected operation left part {left}"),
        }
    }
}

fn read_instructions() -> Vec<Instruction> {
    read_input(2015, 7)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| Instruction::from(line.as_str()))
        .collect()
}

fn run_instructions(instructions: Vec<Instruction>) -> Option<usize> {
    let mut instructions_to_process = VecDeque::from(instructions);

    let mut signals: HashMap<String, usize> = HashMap::new();
    signals.insert("1".into(), 1);

    while !instructions_to_process.is_empty() {
        let mut remaining_instructions_to_process = VecDeque::new();

        for instruction in instructions_to_process {
            match instruction.operation.clone() {
                Operation::WireValue(w) => {
                    match signals.get(&w) {
                        Some(w_v) => {
                            signals.insert(instruction.target_wire.clone(), *w_v);
                        }
                        _ => remaining_instructions_to_process.push_back(instruction),
                    };
                }
                Operation::ValueInt(v) => {
                    signals.insert(instruction.target_wire.clone(), v);
                }
                Operation::And(w1, w2) => {
                    match (signals.get(&w1), signals.get(&w2)) {
                        (Some(w1_v), Some(w2_v)) => {
                            signals.insert(instruction.target_wire.clone(), (w1_v & w2_v) % 65536);
                        }
                        _ => remaining_instructions_to_process.push_back(instruction),
                    };
                }
                Operation::Or(w1, w2) => {
                    match (signals.get(&w1), signals.get(&w2)) {
                        (Some(w1_v), Some(w2_v)) => {
                            signals.insert(instruction.target_wire.clone(), (w1_v | w2_v) % 65536);
                        }
                        _ => remaining_instructions_to_process.push_back(instruction),
                    };
                }
                Operation::Not(w) => {
                    match signals.get(&w) {
                        Some(w_v) => {
                            signals.insert(instruction.target_wire.clone(), (!w_v) % 65536);
                        }
                        _ => remaining_instructions_to_process.push_back(instruction),
                    };
                }
                Operation::Rshift(w, b) => {
                    match signals.get(&w) {
                        Some(w_v) => {
                            signals.insert(instruction.target_wire.clone(), (w_v >> b) % 65536);
                        }
                        _ => remaining_instructions_to_process.push_back(instruction),
                    };
                }
                Operation::Lshift(w, b) => {
                    match signals.get(&w) {
                        Some(w_v) => {
                            signals.insert(instruction.target_wire.clone(), (w_v << b) % 65536);
                        }
                        _ => remaining_instructions_to_process.push_back(instruction),
                    };
                }
            };
        }
        instructions_to_process = remaining_instructions_to_process;
    }

    signals.get("a").copied()
}

pub fn run_part_1() {
    let instructions = read_instructions();
    // generate_dot_file(instructions);

    let signal_a = run_instructions(instructions);
    assert_eq!(Some(46065), signal_a);
}

pub fn run_part_2() {
    let mut instructions = read_instructions();

    for i in instructions.iter_mut() {
        if i.target_wire == "b" {
            match i.operation {
                Operation::ValueInt(_) => i.operation = Operation::ValueInt(46065),
                _ => {}
            };
        }
    }

    let signal_a = run_instructions(instructions);
    assert_eq!(Some(14134), signal_a);
}
