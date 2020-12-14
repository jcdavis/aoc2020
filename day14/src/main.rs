extern crate regex;

use std::env;
use std::fs;
use std::collections::HashMap;

use regex::Regex;

fn compute_mask(str: &str) -> i64 {
    let mut mask_1: i64 = 0;
    str.chars().for_each(|c| {
        if c == '1' {
            mask_1 = (mask_1 << 1) | 1;
        } else {
            mask_1 = mask_1 << 1;
        }
    });
    mask_1
}

fn recurse(mem: &mut HashMap<i64, i64>, floating: &Vec<usize>, i: usize, addr: i64, val: i64) {
    if i == floating.len() {
        //println!("writing val {} at addr {}", val, addr);
        mem.insert(addr, val);
    } else {
        let mask = 1 << floating[i];
        recurse(mem, floating, i+1, addr | mask, val);
        recurse(mem, floating, i+1, addr & !mask, val);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mask_regex = Regex::new(r"mask = (.*)").unwrap();
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let mut current_mask_1: i64 = 0;
    let mut current_floating_bits: Vec<usize> = Vec::new();
    let mut memory: HashMap<i64, i64> = HashMap::new();

    input.lines().for_each(|line| {
        mask_regex.captures(line).map(|cap| {
            current_mask_1 = compute_mask(&cap[1]);
            current_floating_bits.clear();
            let chars: Vec<char> = cap[1].chars().collect();
            for i in 0..chars.len() {
                if chars[i] == 'X' {
                    current_floating_bits.push(35-i);
                }
            }
        });
        mem_regex.captures(line).map(|cap| {
            let addr: i64 = cap[1].parse().unwrap();
            let val: i64 = cap[2].parse().unwrap();
            recurse(&mut memory, &current_floating_bits, 0, addr | current_mask_1, val);
        });
    });
    let sum: i64 = memory.iter().map(|t| {
        let (_addr, val) = t;
        *val
    }).sum();

    println!("{}", sum);
}