use std::collections::HashMap;
use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data = _data_file.trim().split(",");

    let mut current_turn = 1;
    let mut last_occurrence: HashMap<u64, u64> = HashMap::new();

    for starter in data {
        last_occurrence.insert(starter.parse().unwrap(), current_turn);
        current_turn += 1;
    }

    let mut last_num = 0;
    loop {
        if current_turn == 30000000 {
            println!("Turn {}: {}", current_turn, last_num);
            break;
        }
        else if current_turn % 5000000 == 0 || current_turn  == 2020 {
            println!("Turn {}: {}", current_turn, last_num);
        }
        match last_occurrence.get(&last_num) {
            Some(v) => {
                let diff = current_turn - v;
                last_occurrence.insert(last_num, current_turn);
                last_num = diff
            }
            None => {
                last_occurrence.insert(last_num, current_turn);
                last_num = 0;
            }
        }
        current_turn += 1;
    }
}
