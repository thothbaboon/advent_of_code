use crate::read_input;

const UP: char = '(';

fn read_entries() -> Vec<char> {
    read_input(2015, 1)
        .unwrap()
        .map_while(Result::ok)
        .last()
        .unwrap()
        .chars()
        .collect()
}

pub fn run_part_1() {
    let entries = read_entries();
    let up_counts = entries.iter().filter(|c| c == &&UP).count();
    let down_counts = entries.len() - up_counts;
    let floor = up_counts - down_counts;
    assert_eq!(floor, 138);
}

pub fn run_part_2() {
    let entries = read_entries();
    let mut current_floor = 0;
    let mut position = None;

    for (i, entry) in entries.iter().enumerate() {
        current_floor += if entry == &UP { 1 } else { -1 };
        if current_floor == -1 {
            position = Some(i + 1);
            break;
        }
    }

    assert_eq!(position, Some(1771));
}
