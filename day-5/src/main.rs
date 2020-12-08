use std::fs;

fn main() {
    let _example_file = fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data: Vec<&str> = _data_file.trim().lines().collect();

    let mut top_id = 0;
    let mut ids: Vec<i32> = Vec::new();
    for pass in data{
        let row = pass[..7].chars().map(|c| if c == 'B' {1} else {0}).fold(0, |acc, b| acc*2 + b);
        let col = pass[7..].chars().map(|c| if c == 'R' {1} else {0}).fold(0, |acc, b| acc*2 + b);
        let id: i32 = row * 8 + col;
        if id > top_id {
            top_id = id;
        }
        ids.insert(0, id);
    }
    println!("Top id: {}", top_id);
    ids.sort();
    for i in 0..ids.len()-2{
        if ids[i]+1 != ids[i+1]{
            println!("My seat: {}", ids[i]+1);
        }
    }
}