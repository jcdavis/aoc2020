extern crate regex;

use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let re = Regex::new(r"(\d+)-(\d+) (\w): (.*)").unwrap();
    let lines: Vec<(i32, i32, char, String)> = input.lines().map(|line| {
        let mat = re.captures(line).unwrap();
        (mat[1].parse().unwrap(), mat[2].parse().unwrap(), mat[3].chars().next().unwrap(), mat[4].to_string())
    }).collect();

    let mut matching = 0;

    for line in &lines {
        let (min, max, target, str) = line;
        let mut count  = 0;
        str.chars().for_each(|ch| {
            if  &ch == target {
                count+= 1;
            }
        });
        if &count >= min && &count <= max {
            matching += 1;
        }
    }

    println!("{}", matching)
}
