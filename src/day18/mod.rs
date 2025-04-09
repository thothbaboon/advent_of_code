use std::collections::{BinaryHeap, HashMap};

use crate::read_input;

#[derive(Clone)]
struct MemorySpace {
    is_corrupted: bool,
}

impl MemorySpace {
    pub fn to_string(&self) -> String {
        if self.is_corrupted {
            "#".to_string()
        } else {
            ".".to_string()
        }
    }
}

impl std::fmt::Display for MemorySpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

type Memory = Vec<Vec<MemorySpace>>;
type Coordinate = (usize, usize);

#[derive(PartialEq, Eq, Debug)]
struct PathFindingMemorySpace {
    coordinate: Coordinate,
    score: usize,
}

impl Ord for PathFindingMemorySpace {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.coordinate.1.cmp(&other.coordinate.1))
            .then_with(|| self.coordinate.0.cmp(&other.coordinate.0))
    }
}

impl PartialOrd for PathFindingMemorySpace {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Computer {
    memory: Memory,
    fallen_bytes: Vec<Coordinate>,
}

impl Computer {
    pub fn new(memory_size: usize, fallen_bytes: Vec<Coordinate>) -> Self {
        Self {
            memory: vec![
                vec![
                    MemorySpace {
                        is_corrupted: false
                    };
                    memory_size
                ]
                .to_vec();
                memory_size
            ]
            .to_vec(),
            fallen_bytes,
        }
    }

    pub fn run_simulation(&mut self, nb_rounds: usize) {
        for i in 0..nb_rounds {
            self.memory[self.fallen_bytes[i].1][self.fallen_bytes[i].0].is_corrupted = true;
        }
    }

    pub fn find_shortest_path(&self) -> usize {
        let start = PathFindingMemorySpace {
            coordinate: (0, 0),
            score: 0,
        };
        let mut queue = BinaryHeap::new();
        queue.push(start);

        let mut smallest_scores: HashMap<Coordinate, usize> = HashMap::new();

        while let Some(space) = queue.pop() {
            if smallest_scores.contains_key(&space.coordinate) {
                continue;
            }

            if space.coordinate == (MEMORY_SIZE - 1, MEMORY_SIZE - 1) {
                return space.score;
            }

            smallest_scores.insert(space.coordinate, space.score);

            let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

            directions.iter().for_each(|direction| {
                let target = (
                    space.coordinate.0 as isize + direction.0,
                    space.coordinate.1 as isize + direction.1,
                );

                if target.0 < 0
                    || target.1 < 0
                    || target.0 >= MEMORY_SIZE as isize
                    || target.1 >= MEMORY_SIZE as isize
                {
                    return;
                }

                if self.memory[target.0 as usize][target.1 as usize].is_corrupted {
                    return;
                }

                queue.push(PathFindingMemorySpace {
                    coordinate: (target.0 as usize, target.1 as usize),
                    score: space.score + 1,
                })
            });
        }

        panic!("Failed to reach the exit");
    }

    pub fn debug(&self) {
        self.memory.iter().for_each(|row| {
            row.iter().for_each(|space| print!("{}", space));
            println!();
        });
    }
}

fn build_computer() -> Computer {
    let input = read_input("day18", "input.txt")
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| {
            let (s_row, s_col) = line.split_once(",").unwrap();
            (
                s_row.parse::<usize>().unwrap(),
                s_col.parse::<usize>().unwrap(),
            )
        })
        .collect();

    Computer::new(MEMORY_SIZE, input)
}

const MEMORY_SIZE: usize = 71;
const SIMULATION_ROUNDS: usize = 1024;

pub fn run_part_1() {
    let mut computer = build_computer();
    computer.run_simulation(SIMULATION_ROUNDS);
    computer.debug();
    let steps = computer.find_shortest_path();
    println!("{steps}");
}
