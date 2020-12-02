use std::fs;

fn main() {
    let mut n_valid_passwords_unofficial = 0;
    let mut n_valid_passwords_official = 0;
    //let example_database = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

    let passwords = fs::read_to_string("passwords.txt")
        .expect("Something went wrong reading the file");
    let p:Vec<&str> = passwords.split("\n").collect();

    for line in p.iter() {
        let line_split:Vec<&str> = line.split_whitespace().collect();
        let bound_split:Vec<&str> = line_split[0].split("-").collect();
        let bounded_char = line_split[1].chars().next().unwrap();
        let password = line_split[2];

        let min = bound_split[0].parse::<usize>().unwrap();
        let max = bound_split[1].parse::<usize>().unwrap();
        
        let occurences = password.matches(bounded_char).count();
        if occurences >= min && occurences <= max {
            n_valid_passwords_unofficial += 1;
        }

        let val_1 = password.chars().nth(min-1).unwrap();
        let val_2 = password.chars().nth(max-1).unwrap();
        if (val_1 == bounded_char && val_2 != bounded_char) || (val_1 != bounded_char && val_2 == bounded_char) {
            n_valid_passwords_official += 1;
        }
    }
    println!("Number of valid unofficial passwords: {}", n_valid_passwords_unofficial);
    println!("Number of valid official passwords: {}", n_valid_passwords_official);
}
