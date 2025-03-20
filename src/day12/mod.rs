use crate::read_input;
use std::collections::HashSet;

struct Cell {
    label: char,
    already_visited: bool,
}

type Grid = Vec<Vec<Cell>>;

fn init_garden() -> Garden {
    let grid = read_input("day12", "input.txt")
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|label| Cell {
                    label,
                    already_visited: false,
                })
                .collect()
        })
        .collect();

    Garden::new(grid)
}

struct Garden {
    grid: Grid,
}

impl Garden {
    pub fn new(grid: Grid) -> Self {
        Garden { grid }
    }

    fn get_region_fences_price(grid: &mut Grid, r: usize, c: usize) -> usize {
        let mut area = 0;
        let mut perimeter = 0;
        let mut queue = HashSet::new();
        queue.insert((r, c));
        let label = grid[r][c].label;

        while queue.len() > 0 {
            let mut new_queue = HashSet::new();

            for cell in queue {
                area += 1;
                grid[cell.0][cell.1].already_visited = true;

                if cell.0 == 0 {
                    perimeter += 1;
                }
                if cell.1 == 0 {
                    perimeter += 1;
                }
                if cell.0 + 1 == grid.len() {
                    perimeter += 1;
                }
                if cell.1 + 1 == grid[0].len() {
                    perimeter += 1;
                }

                if cell.0 > 0 {
                    if grid[cell.0 - 1][cell.1].label != label {
                        perimeter += 1;
                    } else if !grid[cell.0 - 1][cell.1].already_visited {
                        new_queue.insert((cell.0 - 1, cell.1));
                    }
                }

                if (cell.0 + 1) < grid.len() {
                    if grid[cell.0 + 1][cell.1].label != label {
                        perimeter += 1;
                    } else if !grid[cell.0 + 1][cell.1].already_visited {
                        new_queue.insert((cell.0 + 1, cell.1));
                    }
                }

                if cell.1 > 0 {
                    if grid[cell.0][cell.1 - 1].label != label {
                        perimeter += 1;
                    } else if !grid[cell.0][cell.1 - 1].already_visited {
                        new_queue.insert((cell.0, cell.1 - 1));
                    }
                }

                if (cell.1 + 1) < grid[0].len() {
                    if grid[cell.0][cell.1 + 1].label != label {
                        perimeter += 1;
                    } else if !grid[cell.0][cell.1 + 1].already_visited {
                        new_queue.insert((cell.0, cell.1 + 1));
                    }
                }
            }

            queue = new_queue;
        }

        area * perimeter
    }

    pub fn get_fences_price(&mut self) -> usize {
        let mut price = 0;

        let rows = self.grid.len();
        let cols = self.grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if !self.grid[r][c].already_visited {
                    price += Self::get_region_fences_price(&mut self.grid, r, c);
                }
            }
        }

        price
    }
}

pub fn run_part_1() {
    let mut garden = init_garden();
    println!("{}", garden.get_fences_price());
}
