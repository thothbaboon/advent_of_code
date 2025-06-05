use std::collections::HashSet;

use crate::read_input;

fn read_initial_3d_state() -> HashSet<(isize, isize, isize)> {
    read_input(2020, 17)
        .unwrap()
        .map_while(Result::ok)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some((x as isize, y as isize, 0))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn read_initial_4d_state() -> HashSet<(isize, isize, isize, isize)> {
    read_initial_3d_state()
        .iter()
        .map(|(x, y, z)| (*x, *y, *z, 1))
        .collect()
}

const NB_CYCLES: u32 = 6;

pub fn run_part_1() {
    let mut state = read_initial_3d_state();

    for _ in 0..NB_CYCLES {
        let mut cubes: HashSet<(isize, isize, isize)> = HashSet::<(isize, isize, isize)>::new();

        for active_cube in &state {
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    for z in [-1, 0, 1] {
                        cubes.insert((active_cube.0 + x, active_cube.1 + y, active_cube.2 + z));
                    }
                }
            }
        }

        let mut next_state = HashSet::new();
        for cube in cubes {
            let mut c_active_neighbors = 0;
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    for z in [-1, 0, 1] {
                        if !(x == 0 && y == 0 && z == 0)
                            && state.contains(&(cube.0 + x, cube.1 + y, cube.2 + z))
                        {
                            c_active_neighbors += 1;
                        }
                    }
                }
            }

            let is_active = state.contains(&cube);
            if is_active {
                if c_active_neighbors == 2 || c_active_neighbors == 3 {
                    next_state.insert(cube);
                }
            } else if c_active_neighbors == 3 {
                next_state.insert(cube);
            }
        }

        state = next_state;
    }

    assert_eq!(state.len(), 319);
}

pub fn run_part_2() {
    let mut state = read_initial_4d_state();

    for _ in 0..NB_CYCLES {
        let mut cubes: HashSet<(isize, isize, isize, isize)> =
            HashSet::<(isize, isize, isize, isize)>::new();

        for active_cube in &state {
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    for z in [-1, 0, 1] {
                        for w in [-1, 0, 1] {
                            cubes.insert((
                                active_cube.0 + x,
                                active_cube.1 + y,
                                active_cube.2 + z,
                                active_cube.3 + w,
                            ));
                        }
                    }
                }
            }
        }

        let mut next_state = HashSet::new();
        for cube in cubes {
            let mut c_active_neighbors = 0;
            for x in [-1, 0, 1] {
                for y in [-1, 0, 1] {
                    for z in [-1, 0, 1] {
                        for w in [-1, 0, 1] {
                            if !(x == 0 && y == 0 && z == 0 && w == 0)
                                && state.contains(&(cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w))
                            {
                                c_active_neighbors += 1;
                            }
                        }
                    }
                }
            }

            let is_active = state.contains(&cube);
            if is_active {
                if c_active_neighbors == 2 || c_active_neighbors == 3 {
                    next_state.insert(cube);
                }
            } else if c_active_neighbors == 3 {
                next_state.insert(cube);
            }
        }

        state = next_state;
    }

    assert_eq!(state.len(), 2324);
}
