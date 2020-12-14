use std::collections::HashMap;
use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _example2_file =
        fs::read_to_string("example2.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data: Vec<(&str, &str)> = _data_file
        .trim()
        .lines()
        .map(|i| {
            let mut x = i.split(" = ");
            (x.next().unwrap(), x.next().unwrap())
        })
        .collect();

    part1(&data);
    part2(&data);
}

fn part1(data: &Vec<(&str, &str)>) {
    let mut mask = "";
    let mut mem = HashMap::new();

    for (dest, val) in data {
        if *dest == "mask" {
            mask = val;
        } else {
            let index: usize = dest
                .trim_start_matches("mem[")
                .trim_end_matches("]")
                .parse()
                .unwrap();

            let masked_input = mask
                .chars()
                .zip(format!("{:036b}", val.parse::<u64>().unwrap()).chars())
                .map(|(m, i)| match m {
                    'X' => i.to_digit(10).unwrap() as u64,
                    _ => m.to_digit(10).unwrap() as u64,
                })
                .collect::<Vec<u64>>();
            mem.insert(index, masked_input.clone());
        }
    }
    let sum = mem.values().fold::<u64, _>(0, |sum: u64, bits: &Vec<u64>| {
        sum + bits.iter().fold(0, |acc, &b| acc * 2 + b as u64)
    });

    println!("Part 1: {}", sum);
}

fn part2(data: &Vec<(&str, &str)>) {
    let mut mask = "";
    let mut mem = HashMap::new();

    for (dest, val) in data {
        if *dest == "mask" {
            mask = val;
        } else {
            let index = format!(
                "{:036b}",
                dest.trim_start_matches("mem[")
                    .trim_end_matches("]")
                    .parse::<u64>()
                    .unwrap()
            );

            let masked_index = mask
                .chars()
                .zip(index.chars())
                .map(|(m, i)| match m {
                    '0' => i,
                    _ => m,
                })
                .collect::<String>();
            let addresses = get_addresses(&masked_index[..]);

            for addr in addresses {
                mem.insert(addr, val.parse::<u64>().unwrap());
            }
        }
    }
    let sum = mem.values().fold::<u64, _>(0, |sum: u64, x: &u64| sum + x);

    println!("Part 2: {}", sum);
}

fn get_addresses(addr: &str) -> Vec<usize> {
    match addr.find('X') {
        Some(_) => {
            let mut res = get_addresses(&addr.replacen('X', "0", 1)[..]);
            res.append(&mut get_addresses(&addr.replacen('X', "1", 1)[..]));
            res
        }
        None => vec![
            addr.chars()
                .fold(0, |acc, b| acc * 2 + b.to_digit(10).unwrap() as usize);
            1
        ],
    }
}
