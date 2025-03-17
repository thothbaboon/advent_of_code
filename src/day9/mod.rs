use crate::read_input;

#[derive(Debug)]
struct MovedFile {
    id: usize,
    number: usize,
}

#[derive(Debug)]
struct DiskPart {
    pub file_blocks: usize,
    pub free_space_blocks: usize,
    pub moved: Vec<MovedFile>,
}

fn compute_checksum(files: Vec<DiskPart>) -> usize {
    let mut m = 0;
    let mut checksum = 0;

    for (id, file) in files.iter().enumerate() {
        for _ in 0..file.file_blocks {
            checksum += m * id;
            m += 1;
        }

        for moved in file.moved.iter() {
            for _ in 0..moved.number {
                checksum += m * moved.id;
                m += 1;
            }
        }
    }

    checksum
}

pub fn run_part_1() {
    let lines = read_input("day9", "input.txt").unwrap();
    let mut line: String = lines
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("");
    line.push('0');

    let mut files: Vec<_> = line
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|pair| DiskPart {
            file_blocks: pair[0].to_digit(10).expect("") as usize,
            free_space_blocks: pair[1].to_digit(10).expect("") as usize,
            moved: Default::default(),
        })
        .collect();

    let mut i = 0;
    let mut j = files.len() - 1;
    while i < j {
        if files[i].free_space_blocks >= files[j].file_blocks {
            let number_moved = files[j].file_blocks;
            files[i].free_space_blocks -= number_moved;
            files[i].moved.push(MovedFile {
                id: j,
                number: number_moved,
            });
            files[j].file_blocks = 0;

            j -= 1;
            if files[i].free_space_blocks == 0 {
                i += 1;
            }
        } else {
            let number_moved = files[i].free_space_blocks;
            files[i].free_space_blocks = 0;
            files[j].file_blocks -= number_moved;
            files[i].moved.push(MovedFile {
                id: j,
                number: number_moved,
            });

            i += 1;
        }
    }

    println!("{}", compute_checksum(files));
}
