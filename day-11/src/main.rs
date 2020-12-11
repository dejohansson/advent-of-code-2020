use std::fs;

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data: Vec<Vec<char>> = _data_file
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    println!("Answer 1: {}", problem_1(data.clone()));
    println!("Answer 2: {}", problem_2(data.clone()));
}

fn problem_1(mut seats: Vec<Vec<char>>) -> u32 {
    let mut changed = true;
    let mut next_seats: Vec<Vec<char>> = seats.clone();

    while changed {
        changed = false;
        for (i, row) in seats.iter().enumerate() {
            'iter_seat: for (j, s) in row.iter().enumerate() {
                if *s == 'L' {
                    for (x, y) in &DIRECTIONS {
                        let a_x = (i as isize) + *x;
                        let a_y = (j as isize) + *y;
                        if a_x < 0
                            || a_x >= seats.len() as isize
                            || a_y < 0
                            || a_y >= seats[0].len() as isize
                        {
                            continue;
                        }
                        if seats[a_x as usize][a_y as usize] == '#' {
                            continue 'iter_seat;
                        }
                    }
                    next_seats[i][j] = '#';
                    changed = true;
                }
                if *s == '#' {
                    let mut occupied_count = 0;
                    for (x, y) in &DIRECTIONS {
                        let a_x = (i as isize) + *x;
                        let a_y = (j as isize) + *y;
                        if a_x < 0
                            || a_x >= seats.len() as isize
                            || a_y < 0
                            || a_y >= seats[0].len() as isize
                        {
                            continue;
                        }
                        if seats[a_x as usize][a_y as usize] == '#' {
                            occupied_count += 1;
                            if occupied_count >= 4 {
                                next_seats[i][j] = 'L';
                                changed = true;
                                continue 'iter_seat;
                            }
                        }
                    }
                }
            }
        }
        seats = next_seats.clone();
    }

    let mut num_occupied = 0;
    for r in &seats {
        for s in r {
            if *s == '#' {
                num_occupied += 1;
            }
        }
    }
    num_occupied
}

fn problem_2(mut seats: Vec<Vec<char>>) -> u32 {
    let mut changed = true;
    let mut next_seats: Vec<Vec<char>> = seats.clone();

    while changed {
        changed = false;
        for (i, row) in seats.iter().enumerate() {
            'iter_seat: for (j, s) in row.iter().enumerate() {
                if *s == 'L' {
                    'iter_dir_emp: for (x, y) in &DIRECTIONS {
                        let mut a_x;
                        let mut a_y;
                        let mut steps = 1;
                        while {
                            a_x = (i as isize) + *x * steps;
                            a_y = (j as isize) + *y * steps;
                            if a_x < 0
                                || a_x >= seats.len() as isize
                                || a_y < 0
                                || a_y >= seats[0].len() as isize
                            {
                                continue 'iter_dir_emp;
                            }
                            steps += 1;
                            seats[a_x as usize][a_y as usize] == '.'
                        } {}
                        if seats[a_x as usize][a_y as usize] == '#' {
                            continue 'iter_seat;
                        }
                    }
                    next_seats[i][j] = '#';
                    changed = true;
                }
                if *s == '#' {
                    let mut occupied_count = 0;
                    'iter_dir_occ: for (x, y) in &DIRECTIONS {
                        let mut a_x;
                        let mut a_y;
                        let mut steps = 1;
                        while {
                            a_x = (i as isize) + *x * steps;
                            a_y = (j as isize) + *y * steps;
                            if a_x < 0
                                || a_x >= seats.len() as isize
                                || a_y < 0
                                || a_y >= seats[0].len() as isize
                            {
                                continue 'iter_dir_occ;
                            }
                            steps += 1;
                            seats[a_x as usize][a_y as usize] == '.'
                        } {}
                        if seats[a_x as usize][a_y as usize] == '#' {
                            occupied_count += 1;
                            if occupied_count >= 5 {
                                next_seats[i][j] = 'L';
                                changed = true;
                                continue 'iter_seat;
                            }
                        }
                    }
                }
            }
        }
        seats = next_seats.clone();
    }

    let mut num_occupied = 0;
    for r in &seats {
        for s in r {
            if *s == '#' {
                num_occupied += 1;
            }
        }
    }
    num_occupied
}
