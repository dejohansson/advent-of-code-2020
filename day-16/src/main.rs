use lazy_static::lazy_static;
use std::fs;

use regex::Regex;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let mut data = _data_file.trim().split("\r\n\r\n");

    lazy_static! {
        static ref RULE_REGEX: Regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    let rules: Vec<(&str, u64, u64, u64, u64)> = RULE_REGEX
        .captures_iter(data.next().unwrap())
        .map(|g| {
            (
                g.get(1).unwrap().as_str(),
                g.get(2).unwrap().as_str().parse().unwrap(),
                g.get(3).unwrap().as_str().parse().unwrap(),
                g.get(4).unwrap().as_str().parse().unwrap(),
                g.get(5).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();
    let my_ticket = data.next();


    let mut error_rate = 0;
    for ticket in data.next().unwrap().lines().skip(1).map(|l| {
        l.split(",")
            .map(|d| d.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }) {
        'outer: for num in ticket {
            for rule in &rules{
                if (num >= rule.1 && num <= rule.2) || (num >= rule.3 && num <= rule.4){
                    continue 'outer;
                }
            }
            error_rate += num;
        }
    }

    println!("Ticket scanning error rate: {}", error_rate);
    // for r in rules {
    //     println!("{}  {}  {}  {}  {}", r.0, r.1, r.2, r.3, r.4);
    // }
}
