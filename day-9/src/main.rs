use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data: Vec<u64> = _data_file
        .trim()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let first_invalid = get_first_invalid(&data, 25);
    println!("First invalid: {}", first_invalid);

    let encryption_weakness = get_encrypt_weakness(&data, first_invalid);
    println!("Encryption weakness: {}", encryption_weakness);
}

fn get_first_invalid(data: &Vec<u64>, offset: usize) -> u64 {
    'outer: for (i, num) in data.iter().enumerate().skip(offset) {
        let slice = &data[i - offset..i];
        for (j, x) in slice.iter().enumerate() {
            for y in &slice[j..] {
                if x + y == *num {
                    continue 'outer;
                }
            }
        }
        return *num;
    }
    unreachable!();
}

fn get_encrypt_weakness(data: &Vec<u64>, target: u64) -> u64 {
    for i in 0..data.len() - 2 {
        for j in i + 1..data.len() - 1 {
            let set = &data[i..j];
            let sum = set.iter().sum::<u64>();
            if sum == target {
                return set.iter().min().unwrap() + set.iter().max().unwrap();
            }
        }
    }
    unreachable!();
}
