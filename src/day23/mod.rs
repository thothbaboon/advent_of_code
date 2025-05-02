use std::collections::{HashMap, HashSet};

use crate::read_input;

fn build_graph() -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    read_input("day23", "input.txt")
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (n1, n2) = line.split_once("-").unwrap();
            (n1.to_string(), n2.to_string())
        })
        .for_each(|(n1, n2)| {
            graph.entry(n1.clone()).or_default().insert(n2.clone());
            graph.entry(n2).or_default().insert(n1);
        });

    graph
}

pub fn run_part_1() {
    let graph = build_graph();

    let mut unique_sets = HashSet::new();

    for a in graph.keys() {
        if let Some(a_children) = graph.get(a) {
            for b in a_children {
                if let Some(b_children) = graph.get(b) {
                    for c in b_children {
                        if let Some(c_children) = graph.get(c) {
                            if c_children.contains(a) {
                                let mut set = [a, b, c].to_vec();
                                set.sort();
                                unique_sets.insert(set);
                            }
                        }
                    }
                }
            }
        }
    }

    let count = unique_sets
        .iter()
        .filter(|set| set.iter().any(|v| v.starts_with("t")))
        .count();

    println!("{}", count);
}
