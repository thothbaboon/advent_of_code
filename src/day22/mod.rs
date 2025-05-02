use crate::read_input;

fn read_initial_secret_numbers() -> Vec<usize> {
    read_input("day22", "input.txt")
        .unwrap()
        .map_while(Result::ok)
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn generate_next_secret(secret: usize) -> usize {
    let mut s1 = secret << 6;
    s1 = s1 ^ secret;
    s1 = s1 & ((1 << 24) - 1);

    let mut s2 = s1 >> 5;
    s2 = s2 ^ s1;
    s2 = s2 & ((1 << 24) - 1);

    let mut s3 = s2 << 11;
    s3 = s3 ^ s2;
    s3 = s3 & ((1 << 24) - 1);

    s3
}

fn generate_next_secrets(initial_secret: usize, nb: usize) -> usize {
    let mut secret = initial_secret;
    for _ in 0..nb {
        secret = generate_next_secret(secret);
    }
    secret
}

const PART_1_NB_SECRETS: usize = 2000;

pub fn run_part_1() {
    let initial_secret_numbers = read_initial_secret_numbers();

    let sum = initial_secret_numbers
        .iter()
        .map(|secret| generate_next_secrets(*secret, PART_1_NB_SECRETS))
        .sum::<usize>();

    println!("{:?}", sum);
}
