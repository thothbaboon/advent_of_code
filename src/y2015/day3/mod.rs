use std::collections::HashSet;

use crate::read_input;

enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::North,
            '>' => Direction::East,
            '<' => Direction::West,
            'v' => Direction::South,
            _ => panic!("Unexpected direction {}", value),
        }
    }
}

fn read_directions() -> Vec<Direction> {
    read_input(2015, 3)
        .unwrap()
        .map_while(Result::ok)
        .flat_map(|line| line.chars().map(Direction::from).collect::<Vec<_>>())
        .collect()
}

fn visit_houses(directions: &[&Direction], visited_houses: &mut HashSet<(i32, i32)>) {
    let mut current_position = (0, 0);
    visited_houses.insert(current_position);

    for direction in directions {
        match direction {
            Direction::North => current_position.1 += 1,
            Direction::East => current_position.0 += 1,
            Direction::West => current_position.0 -= 1,
            Direction::South => current_position.1 -= 1,
        }

        visited_houses.insert(current_position);
    }
}

pub fn run_part_1() {
    let directions = read_directions();
    let mut visited_houses = HashSet::new();

    let all_directions: Vec<&Direction> = directions.iter().collect();
    visit_houses(&all_directions, &mut visited_houses);

    assert_eq!(visited_houses.len(), 2081);
}

pub fn run_part_2() {
    let directions = read_directions();
    let mut visited_houses = HashSet::new();

    let santa_directions: Vec<&Direction> = directions
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .collect();
    let robo_santa_directions: Vec<&Direction> = directions
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, v)| v)
        .collect();

    visit_houses(&santa_directions, &mut visited_houses);
    visit_houses(&robo_santa_directions, &mut visited_houses);

    assert_eq!(visited_houses.len(), 2341);
}
