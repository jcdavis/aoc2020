use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let count = count(&chars, 1, 1) * count(&chars, 3, 1) *
        count(&chars, 5, 1) * count(&chars, 7, 1) * count(&chars, 1, 2);
    println!("{}", count);
}

fn count(chars: &Vec<Vec<char>>, xd: usize, yd: usize) -> i64 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut count: i64 = 0;

    while y < chars.len() {
        if chars[y][x] == '#' {
            count += 1;
        }
        y += yd;
        x = (x + xd) % chars[0].len();
    }
    count
}
