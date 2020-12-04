#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let _example_file = fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _example_2_file = fs::read_to_string("example_2.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    
    let passports = _data_file.trim().split("\r\n\r\n");
    let mut pressent_passports = 0;
    let mut valid_passports = 0;
    for pass in passports {
        let mut pass_fields = HashMap::new();
        for field in pass
            .split("\r\n")
            .map(|s| s.split(" ").collect::<Vec<&str>>())
            .flatten()
            .collect::<Vec<&str>>()
        {
            let field = field.split(":").collect::<Vec<&str>>();
            pass_fields.insert(field[0], field[1]);
        }

        if is_present(pass_fields.keys().copied().collect()) {
            pressent_passports += 1;
            if is_valid(pass_fields) {
                valid_passports += 1;
            }
        }
        
    }
    println!("Pressent Passports: {}", pressent_passports);
    println!("Valid Passports: {}", valid_passports);
}

fn is_present(pass_keys: HashSet<&str>) -> bool {
    let required: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();
    if required.difference(&pass_keys).count() == 0 {
        return true;
    }
    false
}

fn is_valid(pass_fields: HashMap<&str, &str>) -> bool {
    lazy_static! {
        static ref BYR: Regex = Regex::new(r"^(19[2-8][0-9]|199[0-9]|200[0-2])$").unwrap();
        static ref IYR: Regex = Regex::new(r"^(201[0-9]|2020)$").unwrap();
        static ref EYR: Regex = Regex::new(r"^(202[0-9]|2030)$").unwrap();
        static ref HGT: Regex = Regex::new(r"^((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap();
        static ref HCL: Regex = Regex::new(r"^#([a-g]|\d){6}$").unwrap();
        static ref ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    if !(BYR.is_match(pass_fields.get("byr").unwrap())
        && IYR.is_match(pass_fields.get("iyr").unwrap())
        && IYR.is_match(pass_fields.get("iyr").unwrap())
        && EYR.is_match(pass_fields.get("eyr").unwrap())
        && HGT.is_match(pass_fields.get("hgt").unwrap())
        && HCL.is_match(pass_fields.get("hcl").unwrap())
        && ECL.is_match(pass_fields.get("ecl").unwrap())
        && PID.is_match(pass_fields.get("pid").unwrap()))
    {
        return false;
    }
    true
}
