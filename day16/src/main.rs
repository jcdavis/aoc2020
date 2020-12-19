extern crate regex;

use std::env;
use std::fs;
use std::collections::HashMap;

use regex::Regex;
use std::borrow::Borrow;

fn is_valid_for_rule(rules: &Vec<(i32, i32)>, num: i32) -> bool {
    for rule in rules {
        if num >= rule.0 && num <= rule.1 {
            return true;
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let field_re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let mut i: usize = 0;
    let mut fields: HashMap<String, Vec<(i32, i32)>> = HashMap::new();

    loop {
        match field_re.captures(lines[i]) {
            Some(cap) => {
                let mut v: Vec<(i32, i32)> = Vec::new();
                v.push((cap[2].parse().unwrap(), cap[3].parse().unwrap()));
                v.push((cap[4].parse().unwrap(), cap[5].parse().unwrap()));
                fields.insert(cap[1].to_string(), v);
                i += 1;
            }
            None => { break; }
        }
    }
    // skip empty, "your ticket:"
    i += 2;
    let _my_ticket: Vec<i32> = lines[i].split(',').map(|n| n.parse().unwrap()).collect();
    // down to first ticket
    i += 3;
    let mut other_tickets: Vec<Box<Vec<i32>>> = Vec::new();

    while i < lines.len() {
        other_tickets.push(Box::new(lines[i].split(',').map(|n| n.parse().unwrap()).collect()));
        i += 1;
    }

    let mut invalid_values = 0;

    for ticket in other_tickets {
        let b: &Vec<i32> = ticket.borrow();
        for value in b {
            let mut is_valid = false;
            for (_, rules) in &fields {
                if is_valid_for_rule(rules, *value) {
                    is_valid = true;
                }
            }
            if !is_valid {
                invalid_values += *value;
            }
        }
    }
    println!("{}", invalid_values);
}
