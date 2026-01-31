use crate::read_input;

struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl Box {
    fn compute_surface_area(&self) -> usize {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn compute_smallest_side_area(&self) -> usize {
        let mut values = [self.l, self.h, self.w];
        values.sort();
        values[0] * values[1]
    }

    fn compute_ribbon_wrap(&self) -> usize {
        (self.l + self.h).min(self.l + self.w).min(self.w + self.h) * 2
    }

    fn compute_ribon_bow(&self) -> usize {
        self.l * self.h * self.w
    }
}

fn read_boxes() -> Vec<Box> {
    read_input(2015, 2)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            // println!("{:?}", line.split('x'));
            let sides: Vec<usize> = line.split('x').map(|side| side.parse().unwrap()).collect();
            Box {
                l: sides[0],
                w: sides[1],
                h: sides[2],
            }
        })
        .collect()
}

pub fn run_part_1() {
    let total: usize = read_boxes()
        .into_iter()
        .map(|b| b.compute_surface_area() + b.compute_smallest_side_area())
        .sum();
    assert_eq!(total, 1586300);
}

pub fn run_part_2() {
    let total: usize = read_boxes()
        .into_iter()
        .map(|b| b.compute_ribbon_wrap() + b.compute_ribon_bow())
        .sum();
    assert_eq!(total, 3737498);
}
