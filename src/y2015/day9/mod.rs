use crate::read_input;

fn read_lines() -> Vec<(String, usize)> {
    read_input(2015, 9)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let (path, value_str) = line.split_once(" = ").unwrap();
            (path.to_string(), value_str.parse::<usize>().unwrap())
        })
        .collect()
}

// Traveling salesman problem
// Solved on paper, input was simple

pub fn run_part_1() {
    let mut lines = read_lines();
    lines.sort_by_key(|v| v.1);

    for line in lines {
        println!("{:?} {:?}", line.0, line.1);
    }

    // 251
}

pub fn run_part_2() {
    let mut lines = read_lines();
    lines.sort_by_key(|v| v.1);
    lines.reverse();

    for line in lines {
        println!("{:?} {:?}", line.0, line.1);
    }

    // 898
}
