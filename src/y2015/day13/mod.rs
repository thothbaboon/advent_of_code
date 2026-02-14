use itertools::Itertools;
use std::collections::HashMap;

use crate::read_input;

type Attendee = String;
type HappinessUnits = isize;
type HappinessMap = HashMap<Attendee, HashMap<Attendee, HappinessUnits>>;

fn get_happiness_units(
    hpm: &HappinessMap,
    attendee: &Attendee,
    neighbour: &Attendee,
) -> HappinessUnits {
    *hpm.get(attendee)
        .unwrap()
        .get(neighbour)
        .expect("There must be a happiness units")
}

fn read_happiness_map() -> HappinessMap {
    let lines = read_input(2015, 13)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            (
                parts[0].to_string(),
                parts[2].to_string(),
                parts[3].parse::<isize>().unwrap(),
                parts[10].replace(".", "").to_string(),
            )
        })
        .collect::<Vec<_>>();

    let mut hpm: HappinessMap = HashMap::new();

    for line in lines {
        let units: isize = if line.1 == "lose" {
            -1 * line.2
        } else {
            line.2
        };

        hpm.entry(line.0).or_default().insert(line.3, units);
    }

    hpm
}

fn calculate_units_part_1(permutation: &[String], hpm: &HappinessMap) -> isize {
    (0..permutation.len())
        .map(|i| {
            let left_i = if i == 0 { 7 } else { i - 1 };
            let right_i = if i == 7 { 0 } else { i + 1 };

            get_happiness_units(hpm, &permutation[i], &permutation[left_i])
                + get_happiness_units(hpm, &permutation[i], &permutation[right_i])
        })
        .sum()
}

// find the lowest happiness units between 2 attendees
// just sit there in between: will be 0 instead, this is where it has the lowest impact
fn calculate_units_part_2(permutation: &[String], hpm: &HappinessMap) -> isize {
    let mut between_2_units = (0..permutation.len())
        .map(|i| {
            let right_i = if i == 7 { 0 } else { i + 1 };

            (
                get_happiness_units(hpm, &permutation[i], &permutation[right_i]),
                get_happiness_units(hpm, &permutation[right_i], &permutation[i]),
            )
        })
        .collect::<Vec<_>>();

    // sit where units are the lowest between 2 attendees
    let min_pos = between_2_units
        .iter()
        .position_min_by_key(|v| v.0 + v.1)
        .unwrap();
    between_2_units[min_pos].0 = 0;
    between_2_units[min_pos].1 = 0;

    between_2_units.iter().map(|v| v.0 + v.1).sum()
}

fn run(calculate_units: fn(perm: &[String], lines: &HappinessMap) -> isize) -> isize {
    let hpm = read_happiness_map();
    let attendees = hpm
        .keys()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let max_happiness_units = attendees
        .clone()
        .into_iter()
        .permutations(attendees.len())
        .map(|perm| calculate_units(&perm, &hpm))
        .max()
        .unwrap();

    max_happiness_units
}

pub fn run_part_1() {
    let result = run(calculate_units_part_1);
    assert_eq!(result, 709);
}

pub fn run_part_2() {
    let result = run(calculate_units_part_2);
    assert_eq!(result, 668);
}
