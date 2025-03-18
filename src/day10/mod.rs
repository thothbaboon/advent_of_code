/*

0 [(0, 3)] -> trailhead
1 [(1, 3)]
2 [(2, 3)]
3 [(3, 3)]
4 [(3, 2), (3, 4)]
5 [(3, 1), (3, 5)]
*/

use std::collections::HashSet;

use crate::read_input;

struct Map(Vec<Vec<usize>>);

impl Map {
    fn get_trailhead_score(&self, r: usize, c: usize) -> usize {
        let mut elevation = 0;
        let mut positions = HashSet::from_iter(vec![(r, c)]);

        while positions.len() > 0 && elevation < 9 {
            elevation += 1;
            let mut next_positions = HashSet::new();

            for position in positions {
                // top
                if position.0 > 0 && self.0[position.0 - 1][position.1] == elevation {
                    next_positions.insert((position.0 - 1, position.1));
                }

                // bottom
                if position.0 < (self.0.len() - 1)
                    && self.0[position.0 + 1][position.1] == elevation
                {
                    next_positions.insert((position.0 + 1, position.1));
                }

                // left
                if position.1 > 0 && self.0[position.0][position.1 - 1] == elevation {
                    next_positions.insert((position.0, position.1 - 1));
                }

                // right
                if position.1 < (self.0[0].len() - 1)
                    && self.0[position.0][position.1 + 1] == elevation
                {
                    next_positions.insert((position.0, position.1 + 1));
                }
            }

            positions = next_positions;
        }

        positions.len()
    }

    pub fn get_trailheads_scores(&self) -> usize {
        let mut score = 0;

        for r in 0..self.0.len() {
            for c in 0..self.0[r].len() {
                if self.0[r][c] == 0 {
                    score += self.get_trailhead_score(r, c);
                }
            }
        }

        score
    }
}

fn read_map() -> Map {
    let grid = read_input("day10", "input.txt")
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|n| n as usize).expect(""))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    Map(grid)
}

pub fn run_part_1() {
    let map = read_map();
    let score = map.get_trailheads_scores();
    println!("{score}");
}
