use crate::read_input;

fn read_patterns_and_designs() -> (Vec<String>, Vec<String>) {
    let lines: Vec<String> = read_input("day19", "input.txt")
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let patterns = lines[0].split(", ").map(|s| s.to_string()).collect();
    let designs = lines[2..].iter().map(|s| s.to_string()).collect();

    (patterns, designs)
}

fn get_is_design_possible(i: usize, design: &str, patterns: &Vec<String>) -> bool {
    if i >= design.len() {
        return true;
    }

    'pattern_loop: for pattern in patterns {
        let mut j = i;
        for c in pattern.chars() {
            if j >= design.len() {
                continue 'pattern_loop;
            }
            if design.as_bytes()[j] != c as u8 {
                continue 'pattern_loop;
            }

            j += 1;
        }

        if get_is_design_possible(j, design, patterns) {
            return true;
        }
    }

    false
}

pub fn run_part_1() {
    let (patterns, designs) = read_patterns_and_designs();

    let possible_designs = designs
        .iter()
        .filter(|design| get_is_design_possible(0, design, &patterns))
        .count();

    println!("{possible_designs}");
}
