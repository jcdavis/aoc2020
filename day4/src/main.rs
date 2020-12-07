extern crate regex;

use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let re = Regex::new(r"(.+)").unwrap();

    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    let mut current: HashMap<String, String> = HashMap::new();

    input.lines().for_each(|line| {
        match re.captures(line) {
            Some(capture) => {
                capture[1].split_ascii_whitespace().for_each(|part| {
                   let mut split = part.split(":");
                    current.insert(split.next().unwrap().to_string(), split.next().unwrap().to_string());
                });
            }
            None => {
                passports.push(current.clone());
                current = HashMap::new();
            }
        };
    });
    passports.push(current);

    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut count = 0;
    for passport in passports {
        let mut valid = true;
        for field in &fields {
            if !passport.contains_key(*field) {
                valid = false;
            }
        }
        if valid {
            count += 1;
        }
    }

    println!("{}", count);
}
