use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data = _data_file.trim().lines();

    let mut current_dir = 'E';
    let mut naive_pos_north = 0;
    let mut naive_pos_east = 0;

    let mut ship_pos = (0, 0);
    let mut waypoint_pos = (1, 10);

    for l in data {
        let mut cs = l.chars();
        let instr = cs.next().unwrap();
        let arg = cs.collect::<String>().parse::<i32>().unwrap();
        println!("{} {}", instr, arg);
        run_naive_instr(
            instr,
            arg,
            &mut current_dir,
            &mut naive_pos_north,
            &mut naive_pos_east,
        );
        run_instr(instr, arg, &mut ship_pos, &mut waypoint_pos);
        println!("{}  {}  {}", current_dir, naive_pos_north, naive_pos_east);
        println!(
            "({}, {})  ({}, {})\n",
            ship_pos.0, ship_pos.1, waypoint_pos.0, waypoint_pos.1
        );
    }

    println!(
        "Naive manhattan distancee: {}",
        naive_pos_north.abs() + naive_pos_east.abs()
    );
    println!(
        "Actual anhattan distnace: {}",
        ship_pos.0.abs() + ship_pos.1.abs()
    );
}

fn run_naive_instr(
    instr: char,
    arg: i32,
    current_dir: &mut char,
    pos_north: &mut i32,
    pos_east: &mut i32,
) {
    match instr {
        'N' => *pos_north += arg,
        'S' => *pos_north -= arg,
        'E' => *pos_east += arg,
        'W' => *pos_east -= arg,
        'F' => run_naive_instr(*current_dir, arg, current_dir, pos_north, pos_east),
        'L' | 'R' => turn_ship(instr, arg, current_dir),
        _ => panic!("Failed to parse instruction!"),
    }
}

fn turn_ship(instr: char, arg: i32, current_dir: &mut char) {
    let mut angle = match *current_dir {
        'N' => 0,
        'S' => 180,
        'E' => 90,
        'W' => 270,
        _ => panic!("Failed to parse current direction!"),
    };

    angle = (angle + arg * if instr == 'L' { -1 } else { 1 }) % 360;
    if angle < 0 {
        angle += 360;
    }

    match angle {
        0 => *current_dir = 'N',
        180 => *current_dir = 'S',
        90 => *current_dir = 'E',
        270 => *current_dir = 'W',
        _ => println!("Failed to parse angle {}!", angle),
    }
}

fn run_instr(instr: char, arg: i32, ship_pos: &mut (i32, i32), waypoint_pos: &mut (i32, i32)) {
    match instr {
        'N' => waypoint_pos.0 += arg,
        'S' => waypoint_pos.0 -= arg,
        'E' => waypoint_pos.1 += arg,
        'W' => waypoint_pos.1 -= arg,
        'F' => {
            *ship_pos = (
                ship_pos.0 + waypoint_pos.0 * arg,
                ship_pos.1 + waypoint_pos.1 * arg,
            )
        }
        'L' | 'R' => turn_waypoint(instr, arg, waypoint_pos),
        _ => panic!("Failed to parse instruction!"),
    }
}

fn turn_waypoint(instr: char, arg: i32, waypoint_pos: &mut (i32, i32)) {
    let mut angle = (arg * if instr == 'L' { -1 } else { 1 }) % 360;
    if angle < 0 {
        angle += 360;
    }

    match angle {
        0 => (),
        90 => *waypoint_pos = (waypoint_pos.1 * -1, waypoint_pos.0),
        180 => *waypoint_pos = (waypoint_pos.0 * -1, waypoint_pos.1 * -1),
        270 => *waypoint_pos = (waypoint_pos.1, waypoint_pos.0 * -1),
        _ => println!("Failed to parse angle {}!", angle),
    }
}
