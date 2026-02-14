use std::collections::HashMap;

use crate::read_input;

struct Reindeer {
    name: String,
    kms_per_sec: usize,
    flying_seconds: usize,
    rest_seconds: usize,
}

fn read_reindeers() -> Vec<Reindeer> {
    read_input(2015, 14)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            Reindeer {
                name: parts[0].to_string(),
                kms_per_sec: parts[3].parse().unwrap(),
                flying_seconds: parts[6].parse().unwrap(),
                rest_seconds: parts[13].parse().unwrap(),
            }
        })
        .collect()
}

const TOTAL_SECONDS: usize = 2503;

fn compute_distance(reindeer: &Reindeer) -> usize {
    let cycle_seconds = reindeer.flying_seconds + reindeer.rest_seconds;
    let complete_cycles: usize = TOTAL_SECONDS / cycle_seconds;

    let flying_seconds_incomplete_cycle =
        reindeer.flying_seconds.min(TOTAL_SECONDS % cycle_seconds);

    let total_flying_seconds =
        flying_seconds_incomplete_cycle + complete_cycles * reindeer.flying_seconds;

    total_flying_seconds * reindeer.kms_per_sec
}

pub fn run_part_1() {
    let result = read_reindeers()
        .iter()
        .map(|r| compute_distance(r))
        .max()
        .unwrap();
    assert_eq!(result, 2696);
}

pub fn run_part_2() {
    let reindeers = read_reindeers();
    let mut leaderboard: HashMap<String, usize> = HashMap::new();
    let mut distances: HashMap<String, usize> = HashMap::new();

    for t in 0..TOTAL_SECONDS {
        for reindeer in reindeers.iter() {
            let cycle_seconds = reindeer.flying_seconds + reindeer.rest_seconds;
            let current_cycle = t % cycle_seconds;
            // Check if flying
            if current_cycle < reindeer.flying_seconds {
                *distances.entry(reindeer.name.clone()).or_default() += reindeer.kms_per_sec;
            }
        }

        let max_dist = distances.values().max().unwrap();
        for (name, distance) in &distances {
            if distance == max_dist {
                *leaderboard.entry(name.clone()).or_default() += 1;
            }
        }
    }

    assert_eq!(*leaderboard.values().max().unwrap(), 1084);
}
