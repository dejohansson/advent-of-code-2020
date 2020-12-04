use std::fs;

fn main() {
    let _example_file = fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data = _example_file.trim();
}
