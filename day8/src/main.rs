extern crate regex;

use std::env;
use std::fs;
use std::collections::HashSet;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let re = Regex::new(r"(\w+) ([+-]\d+)").unwrap();

    let instructions: Vec<(String, i32)> = input.lines().map(|line| {
        let c = re.captures(line).unwrap();
        let val = c[2].parse::<i32>().unwrap();
        (c[1].to_string(), val)
    }).collect();

    let mut visited: HashSet<usize> = HashSet::new();
    let mut current: usize = 0;
    let mut acc = 0;
    loop {
        if visited.contains(&current) {
            break;
        }
        visited.insert(current);
        let num = instructions[current].1;
        match instructions[current].0.as_str() {
            "nop" => {
                current += 1;
            }
            "acc" => {
                acc += num;
                current += 1;
            }
            "jmp" => {
                current = (current as i32 + num) as usize;
            }
            def => {
                panic!("Unexpected instruction {} ", def);
            }
        }
    }
    println!("{}", acc);
}
