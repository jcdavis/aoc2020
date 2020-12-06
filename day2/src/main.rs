extern crate regex;

use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let re = Regex::new(r"(\d+)-(\d+) (\w): (.*)").unwrap();
    let lines: Vec<(usize, usize, char, String)> = input.lines().map(|line| {
        let mat = re.captures(line).unwrap();
        (mat[1].parse::<usize>().unwrap(), mat[2].parse::<usize>().unwrap(), mat[3].chars().next().unwrap(), mat[4].to_string())
    }).collect();

    let mut matching = 0;

    for line in &lines {
        let (min, max, target, str) = line;
        let chars: Vec<char> = str.chars().collect();

        if (chars.get(*min - 1) == Some(target)) ^ (chars.get(*max - 1) == Some(target)) {
            matching += 1
        }
    }

    println!("{}", matching)
}
