use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let mut forms: Vec<HashSet<char>> = Vec::new();

    let mut current: HashSet<char> = HashSet::new();

    input.lines().for_each(|line| {
        if line.len() > 0 {
            line.chars().for_each(|c| {
                current.insert(c);
            });
        } else {
            forms.push(current.clone());
            current.clear();
        }
    });
    forms.push(current);

    let total: usize = forms.iter().map(|f| f.len()).sum();

    println!("{}", total);
}
