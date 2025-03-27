use std::vec;

use crate::read_input;

impl Move {
    const LEFT: Move = Move(0, -1);
    const RIGHT: Move = Move(0, 1);
    const UP: Move = Move(-1, 0);
    const DOWN: Move = Move(1, 0);
}

fn parse_cells(line: &str) -> Vec<Cell> {
    line.chars()
        .map(|c| match c {
            '#' => CellKind::Wall,
            'O' => CellKind::Box,
            _ => CellKind::Empty,
        })
        .map(|kind| Cell { kind })
        .collect::<Vec<_>>()
}

fn parse_moves(line: &str) -> Vec<Move> {
    line.chars()
        .map(|c| match c {
            '<' => Move::LEFT,
            '>' => Move::RIGHT,
            '^' => Move::UP,
            'v' => Move::DOWN,
            invalid => panic!("Invalid direction found while parsing input: {}", invalid),
        })
        .collect::<Vec<_>>()
}

fn init_warehouse() -> Warehouse {
    let mut empty_line_found = false;
    let lines = read_input("day15", "input.txt")
        .unwrap()
        .map_while(Result::ok);

    let mut grid: Vec<Vec<Cell>> = vec![];
    let mut moves: Vec<Move> = vec![];
    let mut robot_position = Position(0, 0);

    for (row, line) in lines.enumerate() {
        if line.is_empty() {
            empty_line_found = true;
            continue;
        }

        if empty_line_found {
            moves.extend(parse_moves(&line));
        } else {
            let robot_index = line.find("@");
            if let Some(col) = robot_index {
                robot_position = Position(row, col);
            }

            grid.push(parse_cells(&line));
        }
    }

    Warehouse {
        grid,
        robot: Robot {
            position: robot_position,
            moves,
        },
    }
}

#[derive(Debug, PartialEq, Eq)]

enum CellKind {
    Wall,
    Box,
    Empty,
}

#[derive(Clone)]
struct Move(isize, isize);

#[derive(Debug)]
struct Position(usize, usize);

impl Position {
    pub fn apply_move(position: &Self, r#move: &Move) -> Position {
        Position(
            (position.0 as isize + r#move.0) as usize,
            (position.1 as isize + r#move.1) as usize,
        )
    }
}

struct Robot {
    position: Position,
    moves: Vec<Move>,
}

#[derive(Debug)]

struct Cell {
    kind: CellKind,
}

struct Warehouse {
    robot: Robot,
    grid: Vec<Vec<Cell>>,
}

impl Warehouse {
    fn gps_coordinates_sum(&self) -> usize {
        let mut s = 0;

        for (row, cells) in self.grid.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                if cell.kind == CellKind::Box {
                    s += 100 * row + col;
                }
            }
        }

        s
    }

    fn try_move_box(&mut self, box_pos: &Position, r#move: &Move) -> bool {
        let target_pos = Position::apply_move(box_pos, r#move);

        match self.grid[target_pos.0][target_pos.1].kind {
            CellKind::Wall => false,
            CellKind::Empty => {
                self.grid[target_pos.0][target_pos.1].kind = CellKind::Box;
                self.grid[box_pos.0][box_pos.1].kind = CellKind::Empty;
                true
            }
            CellKind::Box => {
                if self.try_move_box(&target_pos, r#move) {
                    self.grid[target_pos.0][target_pos.1].kind = CellKind::Box;
                    self.grid[box_pos.0][box_pos.1].kind = CellKind::Empty;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn apply_robot_moves(&mut self) {
        for r#move in self.robot.moves.clone().into_iter() {
            let target_cell_position = Position::apply_move(&self.robot.position, &r#move);
            let target_cell = &self.grid[target_cell_position.0][target_cell_position.1];

            match target_cell.kind {
                CellKind::Wall => continue,
                CellKind::Empty => {
                    self.robot.position = target_cell_position;
                }
                CellKind::Box => {
                    if self.try_move_box(&target_cell_position, &r#move) {
                        self.robot.position = target_cell_position;
                    }
                }
            }
        }
    }
}

pub fn run_part_1() {
    let mut warehouse = init_warehouse();
    warehouse.apply_robot_moves();
    println!("{}", warehouse.gps_coordinates_sum());
}
