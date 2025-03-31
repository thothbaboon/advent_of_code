use lazy_static::lazy_static;
use std::collections::{BinaryHeap, HashMap};

use crate::read_input;

fn parse_maze() -> Vec<Vec<char>> {
    read_input("day16", "input.txt")
        .unwrap()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[derive(Eq, PartialEq)]
struct Node {
    position: Position,
    direction: (isize, isize),
    score: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const EAST: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (0, -1);
const SOUTH: (isize, isize) = (1, 0);
const NORTH: (isize, isize) = (-1, 0);

lazy_static! {
    static ref ROTATE_CLOCKWISE: HashMap<(isize, isize), (isize, isize)> = {
        let mut m = HashMap::new();
        m.insert(NORTH, EAST);
        m.insert(EAST, SOUTH);
        m.insert(SOUTH, WEST);
        m.insert(WEST, NORTH);
        m
    };
    static ref ROTATE_COUNTERCLOCKWISE: HashMap<(isize, isize), (isize, isize)> = {
        let mut m = HashMap::new();
        m.insert(EAST, NORTH);
        m.insert(SOUTH, EAST);
        m.insert(WEST, SOUTH);
        m.insert(NORTH, WEST);
        m
    };
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position(usize, usize);

fn run(
    maze: Vec<Vec<char>>,
    start: Position,
    _end: &Position,
) -> HashMap<(Position, (isize, isize)), usize> {
    let mut queue = BinaryHeap::new();
    queue.push(Node {
        position: start,
        direction: EAST,
        score: 0,
    });

    let mut shortest_distances: HashMap<(Position, (isize, isize)), usize> = HashMap::new();

    while let Some(node) = queue.pop() {
        if shortest_distances.contains_key(&(node.position.clone(), node.direction)) {
            continue;
        }
        shortest_distances.insert((node.position.clone(), node.direction), node.score);

        let next_position = Position(
            (node.position.0 as isize + node.direction.0) as usize,
            (node.position.1 as isize + node.direction.1) as usize,
        );
        if maze[next_position.0][next_position.1] != '#' {
            queue.push(Node {
                position: next_position.clone(),
                direction: node.direction,
                score: node.score + 1,
            });
        }

        queue.push(Node {
            position: node.position.clone(),
            direction: *ROTATE_CLOCKWISE.get(&node.direction).unwrap(),
            score: node.score + 1000,
        });
        queue.push(Node {
            position: node.position,
            direction: *ROTATE_COUNTERCLOCKWISE.get(&node.direction).unwrap(),
            score: node.score + 1000,
        });
    }

    shortest_distances
}

pub fn run_part_1() {
    let maze = parse_maze();
    let start = Position(maze.len() - 2, 1);
    let end = Position(1, maze[0].len() - 2);

    let shortest_distances = run(maze, start, &end);

    let min_distance = [EAST, NORTH, SOUTH, WEST]
        .iter()
        .filter_map(|dir| shortest_distances.get(&(end.clone(), *dir)))
        .min()
        .unwrap();

    println!("{:?}", min_distance);
}
