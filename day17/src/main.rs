use std::env;
use std::fs;
use std::mem;
use std::collections::{HashSet, HashMap};

fn insert_adjencies(pos: &[i32;4], set: &mut HashSet<[i32;4]>, include_self: bool) {
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                for w in -1..=1 {
                    if i != 0 || j != 0 || k != 0 || w != 0 || include_self {
                        set.insert([pos[0] + i, pos[1] + j, pos[2] + k, pos[3] + w]);
                    }
                }
            }
        }
    }
}
fn gen_adjencies(pos: &[i32;4], include_self: bool) -> HashSet<[i32;4]> {
    let mut res: HashSet<[i32;4]> = HashSet::new();
    insert_adjencies(pos, &mut res, include_self);
    res
}

fn next_turn(current: &HashMap<[i32;4], bool>, next: &mut HashMap<[i32;4], bool>) {
    let mut candidates: HashSet<[i32;4]> = HashSet::new();
    for (square, _) in current {
        insert_adjencies(square, &mut candidates, true);
    }
    for m in &candidates {
        let n: i32 = gen_adjencies(m, false).iter().map(|s| -> i32 {
            if current.contains_key(s) {
                1
            } else {
                0
            }
        }).sum();
        if (current.contains_key(m) && n >= 2 && n <= 3) || n == 3 {
            next.insert(*m, true);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut current: HashMap<[i32;4], bool> = HashMap::new();
    let mut next: HashMap<[i32;4], bool> = HashMap::new();

    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        for c in line.chars() {
            if c == '#' {
                current.insert([i, j, 0, 0], true);
            }
            j += 1;
        }
        i += 1;
    }
    for _cycle in 0..6 {
        next_turn(&mut current, &mut next);
        mem::swap(& mut current, &mut next);
        next.clear();
    }
    println!("{}", current.len());
}
