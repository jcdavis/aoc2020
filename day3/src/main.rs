use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut count = 0;

    while y < chars.len() {
        if chars[y][x] == '#' {
            count += 1;
        }
        y += 1;
        x = (x+3) % chars[0].len();
    }
    println!("{}", count);
}
