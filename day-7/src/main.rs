use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _example_file_2 =
        fs::read_to_string("example_2.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let data = _data_file.trim().split("\r\n");

    let mut bag_map: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for line in data {
        let line: Vec<String> = line.split(" contain ").map(|s| s.to_string()).collect();
        let mut inner_bags: Vec<(usize, String)> = Vec::new();
        for bag in line[1].trim_end_matches(".").split(",") {
            let x = bag
                .trim()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            if x[0] != "no" {
                let num_bags = x[0].parse::<usize>().unwrap();
                let inner = x[1..].join(" ").trim_end_matches("s").to_string();
                inner_bags.insert(0, (num_bags, inner));
            }
        }
        let outer_bag = line[0].clone().trim_end_matches("s").to_string();
        bag_map.insert(outer_bag, inner_bags);
    }

    let mut num_contains_gold = 0;
    for (_, val) in bag_map.iter() {
        let visited: HashSet<String> = HashSet::new();
        if contains_gold(val, visited, &bag_map) {
            num_contains_gold += 1;
        }
    }
    println!("Number of bags with gold bag: {}", num_contains_gold);

    let bag_count: HashMap<String, usize> = HashMap::new();
    println!(
        "Number of bags within a gold bag: {}",
        count_bags(bag_map.get("shiny gold bag").unwrap(), bag_count, &bag_map).0
    );
}

fn contains_gold(
    bags: &Vec<(usize, String)>,
    visited: HashSet<String>,
    bag_map: &HashMap<String, Vec<(usize, String)>>,
) -> bool {
    for (_, b) in bags.iter() {
        if b == "shiny gold bag" || b == "shiny gold bags" {
            return true;
        }
        let v = visited.clone();
        if !v.contains(b) {
            let mut w = v.clone();
            w.insert(b.to_string());
            let opt = bag_map.get(b);
            if let Some(x) = opt {
                if contains_gold(x, w, bag_map) {
                    return true;
                }
            }
        }
    }
    false
}

fn count_bags(
    bags: &Vec<(usize, String)>,
    mut bag_count: HashMap<String, usize>,
    bag_map: &HashMap<String, Vec<(usize, String)>>,
) -> (usize, HashMap<String, usize>) {
    let mut sum = 0;
    for (num, bag) in bags.iter() {
        let mut bag_val = 1;
        if bag_count.contains_key(bag) {
            bag_val += *bag_count.get(bag).unwrap();
        } else {
            let opt = bag_map.get(bag);
            if let Some(x) = opt {
                let inner_bags = count_bags(x, bag_count, bag_map);
                bag_count = inner_bags.1;
                bag_val += inner_bags.0;
                bag_count.insert(bag.to_string(), inner_bags.0);
            }
        }
        sum += bag_val * num;
    }
    (sum, bag_count)
}
