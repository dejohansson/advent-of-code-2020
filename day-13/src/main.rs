use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let mut data = _example_file.trim().lines();

    let earliest = data.next().unwrap().parse::<u64>().unwrap();
    let buslines: Vec<&str> = data.next().unwrap().split(",").collect();

    let mut first = (0, u64::MAX);
    for id in buslines
        .iter()
        .filter(|id| **id != "x")
        .map(|id| id.parse::<u64>().unwrap())
    {
        let diff = id - earliest % id;
        if diff < first.1 {
            first = (id, diff);
        }
    }
    println!("Part 1: {}", first.0 * first.1);

    let mut period = 1;
    let mut timestamp = 0;
    for (offset, id) in buslines
        .iter()
        .enumerate()
        .filter(|(_, id)| **id != "x")
        .map(|(offset, id)| (offset, id.parse::<u64>().unwrap()))
    {
        // Start search at previous lower bound and step with previous period
        // until pattern includes the new period.
        timestamp = (0..)
            .map(|x| timestamp + period * x)
            .find(|y| (y + offset as u64) % id == 0)
            .unwrap();
        period = lcm(period, id);
    }
    println!("Part 2: {}", timestamp);
}

fn gcd(x: u64, y: u64) -> u64 {
    match y {
        0 => x,
        _ => gcd(y, x % y),
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}
