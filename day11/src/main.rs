use std::env;
use std::fs;
use std::mem;

fn count_adjacent(map: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut res = 0;

    if i > 0 && j > 0 && map[i-1][j-1] == '#' {
        res += 1;
    }
    if i > 0 && map[i-1][j] == '#' {
        res += 1;
    }
    if i > 0 && map[i-1].get(j+1) == Some(&'#') {
        res += 1;
    }
    if j > 0 && map[i][j-1] == '#' {
        res += 1;
    }
    if map[i].get(j+1) == Some(&'#') {
        res += 1;
    }
    if j > 0 && map.get(i+1).map(|line| line[j-1]) == Some('#') {
        res += 1;
    }
    if map.get(i+1).map(|line| line[j]) == Some('#') {
        res += 1;
    }

    if map.get(i+1).and_then(|line| line.get(j+1)) == Some(&'#') {
        res += 1;
    }
    res
}

fn iterate(current: &mut Vec<Vec<char>>, next: &mut Vec<Vec<char>>) {
    for i in 0..current.len() {
        for j in 0..current[i].len() {
            next[i][j] = current[i][j];
            match current[i][j] {
                'L' if count_adjacent(current, i, j) == 0 => {
                     next[i][j] = '#';
                }
                '#' if count_adjacent(current, i, j) >= 4 => {
                    next[i][j] = 'L';
                }
                _ => {}
            }
        }
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let mut chart: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut next = chart.clone();
    loop {
        iterate(&mut chart, &mut next);
        //print_map(&next);
        if chart == next {
            break;
        }
        mem::swap(&mut chart, &mut next);
    }
    let occupied: usize = next.iter().map(|line| {
        line.iter().filter(|c| **c == '#').count()
    }).sum();
    println!("{}", occupied);
}
