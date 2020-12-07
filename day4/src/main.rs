extern crate regex;

use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn validate(key: &str, s: &str) -> bool {
    let rd = Regex::new(r"^(\d{4})$").unwrap();
    match key {
        "byr" => {
            match rd.captures(s) {
                Some(cap) => {
                    cap[1].parse::<i32>().map(|year| year >= 1920 && year <= 2002).unwrap_or(false)
                }
                None => false
            }
        }
        "iyr" => {
            match rd.captures(s) {
                Some(cap) => {
                    cap[1].parse::<i32>().map(|year| year >= 2010 && year <= 2020).unwrap_or(false)
                }
                None => false
            }
        }
        "eyr" => {
            match rd.captures(s) {
                Some(cap) => {
                    cap[1].parse::<i32>().map(|year| year >= 2020 && year <= 2030).unwrap_or(false)
                }
                None => false
            }
        }
        "hgt" => {
            let hgt = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            match hgt.captures(s) {
                Some(cap) => {
                    cap[1].parse::<i32>().map(|num| match &cap[2] {
                        "cm" => {
                          num >= 150 && num <= 193
                        }
                        "in" => {
                            num >= 59 && num <= 76
                        }
                        _ => false
                    }).unwrap_or(false)
                }
                None => false
            }
        }
        "hcl" => {
            let hcl = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            hcl.is_match(s)
        }
        "ecl" => {
            let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            valid.contains(&s)
        }
        "pid" => {
            let pid = Regex::new(r"^\d{9}$").unwrap();
            pid.is_match(s)
        }
        "cid" => {
            true
        }
        _ => false
    }
}
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
            valid &= passport.get(*field).map(|s| validate(field, s)).unwrap_or(false)
        }
        if valid {
            count += 1;
        }
    }

    println!("{}", count);
}
