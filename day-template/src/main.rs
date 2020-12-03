use std::fs;
use std::io::BufReader;

fn main() {
    let _example_file = BufReader::new(fs::File::open("example.txt").unwrap());
    let _data_file = BufReader::new(fs::File::open("data.txt").unwrap());
}
