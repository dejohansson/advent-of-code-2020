use std::collections::HashSet;
use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data = _data_file.trim().split("\r\n\r\n");
    let mut sum_any = 0;
    let mut sum_all = 0;
    for g in data {
        let mut qs_or: HashSet<char> = HashSet::new();
        let mut qs_and: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        for q in g.lines() {
            let mut qs: HashSet<char> = HashSet::new();
            for c in q.chars() {
                qs.insert(c);
                qs_or.insert(c);
            }
            qs_and = qs.into_iter().filter(|e| qs_and.contains(e)).collect();
        }
        sum_any += qs_or.len();
        sum_all += qs_and.len();
    }
    println!("Sum Any: {}", sum_any);
    println!("Sum All: {}", sum_all);
}
