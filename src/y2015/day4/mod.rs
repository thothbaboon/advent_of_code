use crate::read_input;
use md5;

fn read_secret_key() -> String {
    read_input(2015, 4)
        .unwrap()
        .map_while(Result::ok)
        .last()
        .unwrap()
}

fn find_pattern(pattern: &str) -> usize {
    let secret_key = read_secret_key();

    let mut i = 0;
    let mut digest = "".to_string();
    while !digest.starts_with(pattern) {
        i += 1;
        digest = format!("{:x}", md5::compute(format!("{secret_key}{i}")));
    }

    i
}

pub fn run_part_1() {
    let i = find_pattern("00000");
    assert_eq!(i, 282749);
}

pub fn run_part_2() {
    let i = find_pattern("000000");
    assert_eq!(i, 9962624);
}
