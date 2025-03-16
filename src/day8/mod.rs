use std::collections::{HashMap, HashSet};

use crate::read_input;

pub fn run_part_1() {
    let lines = read_input("day8", "input.txt").unwrap();
    let lines: Vec<_> = lines.map_while(Result::ok).collect();

    let mut antennas_by_label: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for row in 0..lines.len() {
        for (col, ch) in lines[row].chars().enumerate() {
            if ch != '.' {
                antennas_by_label
                    .entry(ch)
                    .or_default()
                    .push((row as isize, col as isize));
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for positions in antennas_by_label.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                let diff_row = p1.0 - p2.0;
                let diff_col = p1.1 - p2.1;
                let p1_antinode = (p1.0 + diff_row, p1.1 + diff_col);
                let p2_antinode = (p2.0 - diff_row, p2.1 - diff_col);

                if p1_antinode.0 >= 0
                    && p1_antinode.1 >= 0
                    && p1_antinode.0 < lines.len().try_into().unwrap()
                    && p1_antinode.1 < lines[0].len().try_into().unwrap()
                {
                    antinodes.insert(p1_antinode);
                }
                if p2_antinode.0 >= 0
                    && p2_antinode.1 >= 0
                    && p2_antinode.0 < lines.len().try_into().unwrap()
                    && p2_antinode.1 < lines[0].len().try_into().unwrap()
                {
                    antinodes.insert(p2_antinode);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
