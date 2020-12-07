use std::env;
use std::fs;

fn to_seat(s: &str) -> (i32, i32) {
    let vec: Vec<char> = s.chars().collect();

    let mut row = 0;
    let mut col = 0;

    for i in 0..7 {
        if vec[i as usize] == 'B' {
            row += 1 << (6-i);
        }
    }
    for i in 7..10 {
        if vec[i as usize] == 'R' {
            col += 1 << (9-i);
        }
    }
    (row, col)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let mut vec: Vec<i32> = input.lines().map(|line| {
        let (row, col) = to_seat(line);
        row*8 + col
    }).collect();

    vec.sort();

    for i in 1..vec.len() {
        if vec[i] - vec[i-1] == 2 {
            println!("{}", vec[i]-1);
        }
    }
}
