use crate::read_input;

const COLS: i32 = 101;
const ROWS: i32 = 103;
const SECONDS: usize = 100;

#[derive(Debug)]
struct Robot {
    row: i32,
    col: i32,
    row_velocity: i32,
    col_velocity: i32,
}

struct BathroomSecurity {
    robots: Vec<Robot>,
}

impl BathroomSecurity {
    pub fn simulate(&mut self, seconds: usize) {
        self.robots.iter_mut().for_each(|robot| {
            let c = robot.col + (seconds as i32 * robot.col_velocity);
            let r = robot.row + (seconds as i32 * robot.row_velocity);

            let mut c = c % COLS;
            let mut r = r % ROWS;

            if c < 0 {
                c += COLS
            };
            if r < 0 {
                r += ROWS
            };

            robot.col = c;
            robot.row = r;
        });
    }

    pub fn print_debug(&self) {
        let mut map = vec![vec![0usize; COLS as usize]; ROWS as usize];

        for robot in self.robots.iter() {
            map[robot.row as usize][robot.col as usize] += 1;
        }

        for row in map {
            let s = row
                .iter()
                .map(|n| {
                    if *n == 0 {
                        ".".to_string()
                    } else {
                        n.to_string()
                    }
                })
                .collect::<String>();
            println!("{}", s);
        }
    }

    pub fn compute_safety_factor(&self) -> usize {
        let cols_first_half = (0, ((COLS - 1) / 2) - 1);
        let cols_second_half = (((COLS - 1) / 2) + 1, COLS - 1);
        let rows_first_half = (0, ((ROWS - 1) / 2) - 1);
        let rows_second_half = (((ROWS - 1) / 2) + 1, ROWS - 1);

        let cols_first_half_range = cols_first_half.0..=cols_first_half.1;
        let cols_second_half_range = cols_second_half.0..=cols_second_half.1;
        let rows_first_half_range = rows_first_half.0..=rows_first_half.1;
        let rows_second_half_range = rows_second_half.0..=rows_second_half.1;

        let mut quadrant_counts = vec![0usize; 4];

        self.robots.iter().for_each(|robot| {
            if rows_first_half_range.contains(&robot.row)
                && cols_first_half_range.contains(&robot.col)
            {
                quadrant_counts[0] += 1;
            } else if rows_first_half_range.contains(&robot.row)
                && cols_second_half_range.contains(&robot.col)
            {
                quadrant_counts[1] += 1;
            } else if rows_second_half_range.contains(&robot.row)
                && cols_first_half_range.contains(&robot.col)
            {
                quadrant_counts[2] += 1;
            } else if rows_second_half_range.contains(&robot.row)
                && cols_second_half_range.contains(&robot.col)
            {
                quadrant_counts[3] += 1;
            }
        });

        quadrant_counts
            .iter()
            .fold(1, |safety_factor, quadrant_count| {
                safety_factor * quadrant_count
            })
    }
}

fn build_bathroom_security() -> BathroomSecurity {
    let robots = read_input("day14", "input.txt")
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (position, velocity) = line.split_once(" ").unwrap();
            let (position, velocity) = (
                position.split_once("=").unwrap().1,
                velocity.split_once("=").unwrap().1,
            );

            let (col, row) = position.split_once(",").unwrap();
            let (col_velocity, row_velocity) = velocity.split_once(",").unwrap();

            Robot {
                row: row.parse::<i32>().unwrap(),
                col: col.parse::<i32>().unwrap(),
                col_velocity: col_velocity.parse::<i32>().unwrap(),
                row_velocity: row_velocity.parse::<i32>().unwrap(),
            }
        })
        .collect();

    BathroomSecurity { robots }
}

pub fn run_part_1() {
    let mut bathroom_security = build_bathroom_security();
    bathroom_security.simulate(SECONDS);
    let safety_factor = bathroom_security.compute_safety_factor();
    // bathroom_security.print_debug();
    println!("{safety_factor}");
}
