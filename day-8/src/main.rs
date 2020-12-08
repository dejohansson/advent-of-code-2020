use std::collections::HashSet;
use std::fs;

fn main() {
    let _example_file =
        fs::read_to_string("example.txt").expect("Something went wrong reading the file");
    let _data_file = fs::read_to_string("data.txt").expect("Something went wrong reading the file");

    let program: Vec<(&str, isize)> = _data_file
        .trim()
        .lines()
        .map(|l| parse_instr(l))
        .collect();

    let mut acc = 0;

    'outer: for i in 0..program.len()-1{
        if program[i].0 == "nop" || program[i].0 == "jmp"{
            let mut test_program = program.clone();
            test_program[i] = (if program[i].0 == "nop" {"jmp"} else {"nop"}, program[i].1);
        
            let mut executed_instructions: HashSet<isize> = HashSet::new();
            let mut ptr: isize = 0;
            acc = 0;

            loop {
                if executed_instructions.contains(&ptr) {
                    break;
                }
                if ptr == test_program.len() as isize{
                    println!("Exited successfully.");
                    break 'outer;
                }

                let instr = test_program[ptr as usize];
                if instr.0 == "acc" {
                    acc += instr.1;
                } else if instr.0 == "jmp" {
                    ptr += instr.1;
                    continue;
                } else if instr.0 == "nop" {
                } else {
                    println!("PANIC at ptr: {}, instr: {} {}", ptr, instr.0, instr.1)
                }

                executed_instructions.insert(ptr);
                ptr += 1;
            }
        }
    }
    println!("Done! \nAcc: {}", acc)
}

fn parse_instr(instr: &str) -> (&str, isize) {
    let split_instr: Vec<&str> = instr.split_whitespace().collect();
    (split_instr[0], split_instr[1].parse::<isize>().unwrap())
}
