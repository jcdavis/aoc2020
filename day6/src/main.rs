use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let mut forms: Vec<Vec<HashSet<char>>> = Vec::new();

    let mut current: Vec<HashSet<char>> = Vec::new();

    input.lines().for_each(|line| {
        if line.len() > 0 {
            let set: HashSet<char> = line.chars().collect();
            current.push(set);
        } else {
            forms.push(current.clone());
            current.clear();
        }
    });
    forms.push(current);

    let total: usize = forms.iter_mut().map(|f| {
        let mut start = f[0].clone();
        for i in 1..f.len() {
            start = f[i].intersection(&start).map(|c| *c).collect();
        }
        start.len()
    }).sum();

    println!("{}", total);
}
