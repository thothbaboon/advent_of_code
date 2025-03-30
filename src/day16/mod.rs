use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    result, usize,
};

use crate::read_input;

#[derive(PartialEq, Eq)]
enum CellKind {
    Empty,
    Wall,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Position(usize, usize);

#[derive(Debug)]
struct Graph(HashMap<Position, Vec<Position>>);

#[derive(PartialEq, Eq, Debug, Clone)]
struct Node {
    position: Position,
    score: usize,
    direction: Option<Direction>,
    from: Option<Position>,
}

impl Node {
    fn new(position: Position) -> Self {
        Self {
            position,
            score: usize::MAX,
            direction: None,
            from: None,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn init_graph() -> (Graph, Position, Position) {
    let input = read_input("day16", "input.txt")
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let start = Position(input.len() - 2, 1);
    let end = Position(1, input[0].len() - 2);

    let grid: Vec<Vec<CellKind>> = input
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| {
                    if c == '#' {
                        CellKind::Wall
                    } else {
                        CellKind::Empty
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut graph = Graph(HashMap::new());

    let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for row in 1..(grid.len() - 1) {
        for col in 1..(grid[row].len() - 1) {
            if grid[row][col] == CellKind::Wall {
                continue;
            }

            let mut neighbors = Vec::new();

            for (dr, dc) in &directions {
                let r = (row as isize + dr) as usize;
                let c = (col as isize + dc) as usize;

                if grid[r][c] == CellKind::Empty {
                    neighbors.push(Position(r, c));
                }
            }

            graph.0.insert(Position(row, col), neighbors);
        }
    }

    (graph, start, end)
}

pub fn run_part_1() {
    let (graph, start, end) = init_graph();
    let results = run_dijkstra(&graph, &start, &end);

    if let Some(result) = results.get(&end) {
        println!("YOLOOOO {:?}", result);
    }
}

fn run_dijkstra(graph: &Graph, start: &Position, end: &Position) -> HashMap<Position, Node> {
    let mut queue = BinaryHeap::new();
    graph.0.keys().for_each(|position| {
        let mut node = Node::new(position.to_owned());
        if position == start {
            node.score = 0;
            node.direction = Some(Direction::East);
        }
        queue.push(node);
    });

    let mut visited = HashSet::new();

    let mut results = HashMap::new();

    while let Some(node) = queue.pop() {
        if visited.contains(&node.position) {
            continue;
        }
        visited.insert(node.position.clone());

        if let Some(neighbors) = graph.0.get(&node.position) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    let (new_direction, rotations) = compute_rotation(
                        &node.position,
                        neighbor,
                        &node.direction.as_ref().unwrap(),
                    );
                    let score = node.score + rotations * 1000 + 1;

                    queue.push(Node {
                        score,
                        direction: Some(new_direction),
                        from: Some(node.position.clone()),
                        position: neighbor.clone(),
                    });
                }
            }
        }

        results.insert(node.position.clone(), node.clone());
    }

    results
}

fn compute_rotation(
    inital: &Position,
    target: &Position,
    direction: &Direction,
) -> (Direction, usize) {
    let pos_change = (
        (target.0 as isize - inital.0 as isize),
        (target.1 as isize - inital.1 as isize),
    );
    let target_direction = match pos_change {
        (0, 1) => Direction::East,
        (0, -1) => Direction::West,
        (1, 0) => Direction::South,
        _ => Direction::North,
    };

    if &target_direction == direction {
        return (target_direction, 0);
    }

    let rotations_order = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    let direction_index = rotations_order.iter().position(|d| d == direction).unwrap() as isize;
    let target_direction_index = rotations_order
        .iter()
        .position(|d| d == &target_direction)
        .unwrap() as isize;

    let clockwise = (target_direction_index - direction_index + 4) % 4;
    let counter_clockwise = (direction_index - target_direction_index + 4) % 4;

    (target_direction, clockwise.min(counter_clockwise) as usize)
}
