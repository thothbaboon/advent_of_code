use crate::read_input;

fn read_puzzle_input() -> String {
    read_input(2015, 10)
        .unwrap()
        .map_while(Result::ok)
        .collect()
}

fn look_and_say(s: &str, runs: usize) -> usize {
    let mut current: Vec<isize> = s.chars().map(|c| c.to_string().parse().unwrap()).collect();

    for _ in 0..runs {
        let mut next = [].to_vec();

        let mut value = current[0];
        let mut count = 1;
        for i in 1..current.len() {
            if current[i] != value {
                next.push(count);
                next.push(value);
                value = current[i];
                count = 1;
            } else {
                count += 1;
            }
        }
        next.push(count);
        next.push(value);

        current = next;
    }

    current.len()
}

pub fn run_part_1() {
    let input = read_puzzle_input();
    let result = look_and_say(&input, 40);
    assert_eq!(result, 360154);
}

pub fn run_part_2() {
    let input = read_puzzle_input();
    let result = look_and_say(&input, 50);
    assert_eq!(result, 5103798);
}
