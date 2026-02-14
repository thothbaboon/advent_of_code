use crate::read_input;

fn read_puzzle_input() -> String {
    read_input(2015, 11)
        .unwrap()
        .map_while(Result::ok)
        .collect()
}

fn increment(password: &mut [u32]) {
    let mut i = password.len();

    while i > 0 {
        i -= 1;

        if password[i] == ('z' as u32) {
            password[i] = 'a' as u32;
        } else {
            password[i] += 1;
            break;
        }
    }
}

fn check_two_overlapping_pairs(password: &[u32]) -> bool {
    let mut found_one = false;

    let mut i = 1;
    while i < password.len() {
        if password[i] == password[i - 1] {
            if found_one {
                return true;
            } else {
                found_one = true;
                i += 1;
            }
        }
        i += 1;
    }

    false
}

fn digits_to_string(digits: &[u32]) -> String {
    digits
        .iter()
        .map(|d| char::from_u32(*d).unwrap())
        .collect::<String>()
}

fn string_to_digits(s: &str) -> Vec<u32> {
    s.chars().map(|c| c as u32).collect()
}

fn check(password: &[u32]) -> bool {
    if password
        .iter()
        .any(|&c| c == 'i' as u32 || c == 'o' as u32 || c == 'l' as u32)
    {
        return false;
    }

    if !check_two_overlapping_pairs(password) {
        return false;
    }

    password
        .windows(3)
        .any(|w| w[2] == w[1] + 1 && w[1] == w[0] + 1)
}

fn get_next_password(password: &mut Vec<u32>) {
    increment(password);

    while !check(&password) {
        increment(password);
    }
}

fn run(password: &str) -> String {
    let mut password_digits: Vec<u32> = string_to_digits(password);
    get_next_password(&mut password_digits);
    digits_to_string(&password_digits)
}

pub fn run_part_1() {
    let password = read_puzzle_input();
    assert_eq!(run(&password), "cqjxxyzz".to_string());
}

pub fn run_part_2() {
    assert_eq!(run("cqjxxyzz"), "cqkaabcc".to_string());
}
