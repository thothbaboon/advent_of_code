use regex::Regex;

use crate::read_input;

fn read_json() -> String {
    read_input(2015, 12)
        .unwrap()
        .map_while(Result::ok)
        .collect()
}

fn count(s: &str) -> i64 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let captures = re.captures_iter(s);

    captures
        .map(|c| c.extract())
        .map(|(_, [v])| v.parse::<i64>().unwrap())
        .sum()
}

pub fn run_part_1() {
    let json = read_json();
    let c = count(&json);
    assert_eq!(c, 156366);
}

pub fn remove_red_objects(s: &str) -> String {
    let all_occurences = s.match_indices(r#":"red""#).collect::<Vec<_>>();
    // Keep original to see the brackets 
    let original = s.chars().collect::<Vec<char>>();
    let mut result = s.chars().collect::<Vec<char>>();

    for (i, _) in all_occurences {
        // erase front
        let mut to_erase = 1;
        let mut current = i;
        while current > 0 && to_erase > 0 {
            current -= 1;
            if original[current] == '{' {
                to_erase -= 1;
            }
            if original[current] == '}' {
                to_erase += 1;
            }
            result[current] = 'x';
        }

        // erase  back
        let mut to_erase = 1;
        let mut current = i;
        while current < (original.len() - 1) && to_erase > 0 {
            current += 1;
            if original[current] == '}' {
                to_erase -= 1;
            }
            if original[current] == '{' {
                to_erase += 1;
            }
            result[current] = 'x';
        }
    }

    result.into_iter().collect()
}

/*
Input observation:
- same count for `red` and `"red"`
- no `"red":` so red is never used as object key
- less `:"red"` than `"red"` because some are array values and not object values
So the idea is to find all `:"red"` and erase the object they are contained in
*/
pub fn run_part_2() {
    let json = read_json();
    let new_json = remove_red_objects(&json);
    let c = count(&new_json);
    assert_eq!(c, 96852);
}
