extern crate regex;

use std::env;
use std::fs;
use std::collections::HashMap;

use regex::Regex;

fn compute_mask(str: &str) -> (i64, i64) {
    let mut mask_0: i64 = 0;
    let mut mask_1: i64 = 0;
    str.chars().for_each(|c| {
        if c == '0' {
            mask_0 = mask_0 << 1;
        } else {
            mask_0 = (mask_0 << 1) | 1;
        }
        if c == '1' {
            mask_1 = (mask_1 << 1) | 1;
        } else {
            mask_1 = mask_1 << 1;
        }
    });
    (mask_0, mask_1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mask_regex = Regex::new(r"mask = (.*)").unwrap();
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let mut current_mask_0: i64 = -1;
    let mut current_mask_1: i64 = 0;
    let mut memory: HashMap<i64, i64> = HashMap::new();

    input.lines().for_each(|line| {
        mask_regex.captures(line).map(|cap| {
            let t = compute_mask(&cap[1]);
            current_mask_0 = t.0;
            current_mask_1 = t.1;
        });
        mem_regex.captures(line).map(|cap| {
            let addr: i64 = cap[1].parse().unwrap();
            let val: i64 = cap[2].parse().unwrap();
            memory.insert(addr, (val & current_mask_0) | current_mask_1);
        });
    });
    let sum: i64 = memory.iter().map(|t| {
        let (_addr, val) = t;
        *val
    }).sum();

    println!("{}", sum);
}