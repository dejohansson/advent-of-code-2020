use std::collections::HashMap;
use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _example2_file =
        fs::read_to_string("example2.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let mut data: Vec<usize> = _data_file
        .trim()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    data.sort();
    data.insert(0, 0);

    let mut num_one_jolt_diff = 0;
    let mut num_three_jolt_diff = 1;

    for i in 1..data.len() {
        if data[i] - data[i - 1] == 1 {
            num_one_jolt_diff += 1;
        } else if data[i] - data[i - 1] == 3 {
            num_three_jolt_diff += 1;
        }
    }
    println!("{}", num_one_jolt_diff * num_three_jolt_diff);

    let mut distinct_arrangements: HashMap<usize, usize> = HashMap::new();
    let num_arrangements = get_num_arrangements(&data, &mut distinct_arrangements, 0);
    println!("{}", num_arrangements);
}

fn get_num_arrangements(
    data: &Vec<usize>,
    arrangements: &mut HashMap<usize, usize>,
    index: usize,
) -> usize {
    if index == data.len() - 1 {
        return 1;
    }
    let mut res = 0;
    let mut next_index = index + 1;
    while next_index < data.len() && data[next_index] <= data[index] + 3 {
        let num;
        match arrangements.get(&next_index) {
            Some(val) => {
                num = *val;
            }
            None => {
                num = get_num_arrangements(&data, arrangements, next_index);
                arrangements.insert(next_index, num);
            }
        }
        res += num;
        next_index += 1;
    }
    res
}
