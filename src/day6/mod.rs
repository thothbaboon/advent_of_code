use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::read_input;

const VISITED_CHAR: char = 'X';
const OBSTACLE_CHAR: char = '#';

#[derive(Hash, PartialEq, Eq, Clone)]
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
    fn mark_cell_as_visited_if_not(&mut self, position: &Position) -> bool {
        if self.0[position.row as usize][position.col as usize] != VISITED_CHAR {
            self.0[position.row as usize][position.col as usize] = VISITED_CHAR;
            return true;
        }
        false
    }

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

fn init_grid() -> Grid {
    let lines = read_input("day6", "input.txt").unwrap();

    Grid(
        lines
            .filter_map(Result::ok)
            .map(|line| line.chars().collect())
            .collect(),
    )
}

fn find_next_valid_move(
    current: &Position,
    direction: Direction,
    grid: &Grid,
) -> Option<(Position, Direction)> {
    let mut current_direction = direction;

    // try the 4 directions around the guard
    for _ in 0..4 {
        let next_position = Position::add(
            &current,
            DIRECTIONS
                .get(&current_direction)
                .expect("No position movement for this direction"),
        );

        if !grid.is_valid_position(&next_position) {
            return None;
        }

        if !grid.is_obstacle(&next_position) {
            return Some((next_position, current_direction));
        }

        current_direction = DIRECTION_ROTATIONS
            .get(&current_direction)
            .expect("No rotation found for direction")
            .clone();
    }

    return None;
}

pub fn run_part_1() {
    let mut grid = init_grid();

    let mut distinct_visited = 0;
    let mut guard_position = grid.find_initial_guard_position();
    let mut guard_direction = Direction::North;

    while grid.is_valid_position(&guard_position) {
        if grid.mark_cell_as_visited_if_not(&guard_position) {
            distinct_visited += 1;
        }

        if let Some((next_position, next_direction)) =
            find_next_valid_move(&guard_position, guard_direction, &grid)
        {
            guard_position = next_position;
            guard_direction = next_direction;
        } else {
            break;
        }
    }

    println!("{distinct_visited}");
}
