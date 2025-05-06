use std::collections::HashMap;

use lazy_static::lazy_static;

use super::dijkstra::keypad_dijkstra;

#[derive(Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::UP => "^",
            Direction::DOWN => "v",
            Direction::RIGHT => ">",
            Direction::LEFT => "<",
        }
    }
}

lazy_static! {
    pub static ref NUMERIC_KEYPAD: HashMap<&'static str, Vec<(Direction, &'static str)>> = {
        let mut m = HashMap::new();

        // Top row
        m.insert("7", vec![(Direction::RIGHT, "8"), (Direction::DOWN, "4")]);
        m.insert("8", vec![(Direction::LEFT, "7"), (Direction::RIGHT, "9"), (Direction::DOWN, "5")]);
        m.insert("9", vec![(Direction::LEFT, "8"), (Direction::DOWN, "6")]);

        // Middle row
        m.insert("4", vec![(Direction::UP, "7"), (Direction::RIGHT, "5"), (Direction::DOWN, "1")]);
        m.insert("5", vec![(Direction::UP, "8"), (Direction::LEFT, "4"), (Direction::RIGHT, "6"), (Direction::DOWN, "2")]);
        m.insert("6", vec![(Direction::UP, "9"), (Direction::LEFT, "5"), (Direction::DOWN, "3")]);

        // Bottom row
        m.insert("1", vec![(Direction::UP, "4"), (Direction::RIGHT, "2")]);
        m.insert("2", vec![(Direction::UP, "5"), (Direction::LEFT, "1"), (Direction::RIGHT, "3"), (Direction::DOWN, "0")]);
        m.insert("3", vec![(Direction::UP, "6"), (Direction::LEFT, "2"), (Direction::DOWN, "A")]);

        // Bottom-most row
        m.insert("0", vec![(Direction::UP, "2"), (Direction::RIGHT, "A")]);
        m.insert("A", vec![(Direction::UP, "3"), (Direction::LEFT, "0")]);

        m
    };

    pub static ref DIRECTIONAL_KEYPAD: HashMap<&'static str, Vec<(Direction, &'static str)>> = {
        let mut m = HashMap::new();

        // Top row
        m.insert("^", vec![(Direction::RIGHT, "A"), (Direction::DOWN, "v")]);
        m.insert("A", vec![(Direction::LEFT, "^"), (Direction::DOWN, ">")]);

        // Bottom row
        m.insert("<", vec![(Direction::RIGHT, "v")]);
        m.insert("v", vec![(Direction::UP, "^"), (Direction::LEFT, "<"), (Direction::RIGHT, ">")]);
        m.insert(">", vec![(Direction::UP, "A"), (Direction::LEFT, "v")]);

        m
    };
}

pub struct Keypad {
    keys: &'static HashMap<&'static str, Vec<(Direction, &'static str)>>,
}

impl Keypad {
    pub fn new(keys: &'static HashMap<&'static str, Vec<(Direction, &'static str)>>) -> Keypad {
        Keypad { keys }
    }

    pub fn find_keypad_shortest_sequences(&self, from: &str, to: &str) -> Vec<String> {
        let mut sequences = keypad_dijkstra(from, to, self.keys);
        // press A after each sequence
        // the sequence moves the arm from `from` to `to` but then need to push the key with `A`
        sequences.iter_mut().for_each(|s| s.push_str("A"));
        sequences
    }
}
