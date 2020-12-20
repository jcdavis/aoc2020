extern crate regex;

use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};

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
    let my_ticket: Vec<i32> = lines[i].split(',').map(|n| n.parse().unwrap()).collect();
    // down to first ticket
    i += 3;
    let mut other_tickets: Vec<Box<Vec<i32>>> = Vec::new();

    while i < lines.len() {
        let ticket: Vec<i32> = lines[i].split(',').map(|n| n.parse().unwrap()).collect();
        let mut is_valid_ticket = true;
        for num in &ticket {
            let mut is_valid_num = false;
            for (_, rules) in &fields {
                if is_valid_for_rule(rules, *num) {
                    is_valid_num = true;
                }
            }
            if !is_valid_num {
                is_valid_ticket = false;
            }
        }
        if is_valid_ticket {
            other_tickets.push(Box::new(ticket));
        }
        i += 1;
    }

    let mut valid_cols_for_field: HashMap<String, HashSet<usize>> = HashMap::new();

    for field in &fields {
        for i in 0..my_ticket.len() {
            let mut is_valid_for_col = true;
            for ticket in &other_tickets {
                let t: &Vec<i32> = ticket.borrow();
                if !is_valid_for_rule(field.1, t[i]) {
                    is_valid_for_col = false;
                }
            }
            if is_valid_for_col {
                valid_cols_for_field.entry(field.0.to_string()).or_insert(HashSet::new()).insert(i);
            }
        }
    }

    let mut placed: HashMap<String, usize> = HashMap::new();

    while valid_cols_for_field.len() > 0 {
        let (field, col) = {
            let mut as_vec: Vec<(&String, &HashSet<usize>)> = valid_cols_for_field.iter().collect();
            as_vec.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
            let (field, colSet) = as_vec.pop().unwrap();
            let col = *colSet.iter().next().unwrap();
            placed.insert(field.to_string(), col);
            (field.to_string(), col)
        };
        valid_cols_for_field.remove(&field);
        for (_, set) in &mut valid_cols_for_field {
            set.remove(&col);
        }
    }

    let mut mult: i64 = 1;
    for (field_name, col) in &placed {
        if field_name.starts_with("departure") {
            mult *= my_ticket[*col] as i64;
        }
    }
    println!("{}", mult);
}
