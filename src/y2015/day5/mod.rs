use std::collections::HashSet;

use crate::read_input;

fn read_strings() -> Vec<String> {
    read_input(2015, 5).unwrap().map_while(Result::ok).collect()
}

fn is_nice(s: &str, vowels: &HashSet<char>, forbidden_substrings: &[&str]) -> bool {
    let vowels_count = s.chars().filter(|c| vowels.contains(c)).count();
    if vowels_count < 3 {
        return false;
    }

    for fb in forbidden_substrings {
        if s.contains(fb) {
            return false;
        }
    }

    let chars: Vec<char> = s.chars().collect();
    for i in 1..s.len() {
        if chars[i - 1] == chars[i] {
            return true;
        }
    }

    false
}

fn has_pair_appearing_twice(s: &str) -> bool {
    for i in 1..s.len() {
        for j in i+2..s.len() {
            if s.get(i-1..=i) == s.get(j-1..=j) {
                return true;
            }
        }
    }

    false
}

fn is_nice_v2(s: &str) -> bool {
    if !has_pair_appearing_twice(s) {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();
    for i in 2..s.len() {
        if chars[i - 2] == chars[i] {
            return true;
        }
    }

    false
}

pub fn run_part_1() {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let forbidden_substrings = ["ab", "cd", "pq", "xy"];
    let c = read_strings()
        .into_iter()
        .filter(|s| is_nice(s, &vowels, &forbidden_substrings))
        .count();
    assert_eq!(c, 238);
}

pub fn run_part_2() {
    let c = read_strings()
        .into_iter()
        .filter(|s| is_nice_v2(s))
        .count();
    assert_eq!(c, 69);
}
