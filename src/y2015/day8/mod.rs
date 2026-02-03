use crate::read_input;

fn read_lines() -> Vec<String> {
    read_input(2015, 8).unwrap().map_while(Result::ok).collect()
}

fn count_memory_chars(s: &str) -> usize {
    let mut count = 0;

    let chars: Vec<char> = s.chars().collect();
    let mut i: usize = 1;

    while i < (s.len() - 1) {
        if chars[i] == '\\' {
            if chars[i + 1] == 'x' {
                count += 1;
                i += 4;
                continue;
            }
            if chars[i + 1] == '"' || chars[i + 1] == '\\' {
                count += 1;
                i += 2;
                continue;
            }
        }
        count += 1;
        i += 1;
    }

    count
}

pub fn run_part_1() {
    let lines = read_lines();
    let total_string_code: usize = lines.iter().map(|l| l.len()).sum();
    let total_memory: usize = lines.iter().map(|l| count_memory_chars(&l)).sum();

    assert_eq!(total_string_code - total_memory, 1350)
}

pub fn run_part_2() {
    let lines = read_lines();
    let total_string_code: usize = lines.iter().map(|l| l.len()).sum();
    let total_new_format: usize = lines
        .iter()
        .map(|l| format!("\"{}\"", l.replace("\\", "\\\\").replace("\"", "\\\"")).len())
        .sum();

    assert_eq!(total_new_format - total_string_code, 2085)
}
