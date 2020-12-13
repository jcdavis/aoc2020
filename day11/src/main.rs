use std::env;
use std::fs;
use std::mem;

fn count_adjacent(map: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut res = 0;
    let deltas = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for delta in deltas {
        let (di, dj) = delta;
        let mut si = i as i32 + di;
        let mut sj = j as i32 + dj;

        while si >= 0 && si < map.len() as i32 && sj >= 0 &&
            sj < map[0].len() as i32 {
            let c = map[si as usize][sj as usize];
            if c == '.' {
                si += di;
                sj += dj;
            } else {
                if c == '#' {
                    res += 1;
                }
                break;
            }
        }
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
                '#' if count_adjacent(current, i, j) >= 5 => {
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
