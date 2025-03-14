use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

use crate::read_input;

const VISITED_CHAR: char = 'X';
const OBSTACLE_CHAR: char = '#';

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

lazy_static! {
    static ref DIRECTIONS: HashMap<Direction, Position> = HashMap::from([
        (Direction::North, (-1, 0).into()),
        (Direction::South, (1, 0).into()),
        (Direction::West, (0, -1).into()),
        (Direction::East, (0, 1).into()),
    ]);
    static ref DIRECTION_ROTATIONS: HashMap<Direction, Direction> = HashMap::from([
        (Direction::North, Direction::East),
        (Direction::East, Direction::South),
        (Direction::South, Direction::West),
        (Direction::West, Direction::North),
    ]);
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Position { row, col }
    }

    pub fn add(p1: &Position, p2: &Position) -> Self {
        Position {
            row: p1.row + p2.row,
            col: p1.col + p2.col,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from((row, col): (i32, i32)) -> Self {
        Position::new(row, col)
    }
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn is_obstacle(&self, position: &Position) -> bool {
        self.0[position.row as usize][position.col as usize] == OBSTACLE_CHAR
    }

    fn find_initial_guard_position(&self) -> Position {
        for r in 0..self.0.len() {
            for c in 0..self.0[r].len() {
                if self.0[r][c] == '^' {
                    return Position {
                        row: r as i32,
                        col: c as i32,
                    };
                }
            }
        }

        panic!("Guard not found on the grid");
    }

    fn is_valid_position(&self, position: &Position) -> bool {
        position.row < self.0.len() as i32
            && position.col < self.0[0].len() as i32
            && position.row >= 0
            && position.col >= 0
    }
}

struct Runner<'a> {
    grid: &'a Grid,
    guard_position: Position,
    guard_direction: Direction,
    pub distinct_visited: i32,
    pub visited_cells: HashMap<Position, HashSet<Direction>>,
}

impl<'a> Runner<'a> {
    pub fn new(grid: &'a Grid,) -> Self {
        Runner {
            grid,
            guard_position: grid.find_initial_guard_position(),
            guard_direction: Direction::North,
            distinct_visited: Default::default(),
            visited_cells: Default::default(),
        }
    }

    fn has_guard_position_been_visited(&self) -> bool {
        self.visited_cells.contains_key(&self.guard_position)
    }

    fn mark_as_visited(&mut self) {
        self.visited_cells
            .entry(self.guard_position)
            .or_default()
            .insert(self.guard_direction);
    }

    fn find_next_valid_move(
        &self,
    ) -> Option<(Position, Direction)> {
        let mut current_direction = self.guard_direction;

        // try the 4 directions around the guard
        for _ in 0..4 {
            let next_position = Position::add(
                &self.guard_position,
                DIRECTIONS
                    .get(&current_direction)
                    .expect("No position movement for this direction"),
            );

            if !self.grid.is_valid_position(&next_position) {
                return None;
            }

            if !self.grid.is_obstacle(&next_position) {
                return Some((next_position, current_direction));
            }

            current_direction = DIRECTION_ROTATIONS
                .get(&current_direction)
                .expect("No rotation found for direction")
                .clone();
        }

        return None;
    }

    pub fn run(&mut self) {
        while self.grid.is_valid_position(&self.guard_position) {
            if !self.has_guard_position_been_visited() {
                self.distinct_visited += 1;
            }
            self.mark_as_visited();

            if let Some((next_position, next_direction)) =
                self.find_next_valid_move()
            {
                self.guard_position = next_position;
                self.guard_direction = next_direction;
            } else {
                break;
            }
        }
    }
}

fn init_grid() -> Grid {
    let lines = read_input("day6", "input.txt").unwrap();

    Grid(
        lines
            .filter_map(Result::ok)
            .map(|line| line.chars().collect())
            .collect(),
    )
}

pub fn run_part_1() {
    let grid = init_grid();

    let mut runner = Runner::new(&grid);
    runner.run();

    println!("{}", runner.distinct_visited);
}
