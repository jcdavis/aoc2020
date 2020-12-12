extern crate regex;

use std::env;
use std::fs;
use std::collections::HashSet;

use regex::Regex;
use std::process::exit;

fn run_program(instructions: &Vec<(String, i32)>) -> Option<i32> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut current: usize = 0;
    let mut acc = 0;
    loop {
        if current == instructions.len() {
            return Some(acc);
        }
        if visited.contains(&current) {
            return None;
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let re = Regex::new(r"(\w+) ([+-]\d+)").unwrap();

    let mut instructions: Vec<(String, i32)> = input.lines().map(|line| {
        let c = re.captures(line).unwrap();
        let val = c[2].parse::<i32>().unwrap();
        (c[1].to_string(), val)
    }).collect();

    for i in 0..instructions.len() {
        let num = instructions[i].1;
        match instructions[i].0.as_str() {
            "nop" => {
                instructions[i] = ("jmp".to_string(), num);
                run_program(&instructions).map(|res| {
                    println!("{}", res);
                    exit(0);
                });
                instructions[i] = ("nop".to_string(), num);
            }
            "jmp" => {
                instructions[i] = ("nop".to_string(), num);
                run_program(&instructions).map(|res| {
                    println!("{}", res);
                    exit(0);
                });
                instructions[i] = ("jmp".to_string(), num);
            }
            _ => { }
        }
    }
}
