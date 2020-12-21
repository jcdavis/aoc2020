use std::env;
use std::fs;

fn apply(l: i64, r: i64, op: char) -> i64 {
    match op {
        '+' => { l + r }
        '*' => { l * r }
        _ => panic!("unknown op {}", op)
    }
}

fn eval<I>(parts: & mut I) -> i64
where
    I: Iterator<Item=char>
{
    let mut current: i64 = 0;
    let mut op = '+';
    loop {
        match parts.next() {
            None => {
                panic!("????");
            }
            Some(p) => {
                let num = match p.to_digit(10) {
                    Some(d) => { d as i64 }
                    None => {
                        assert!(p == '(');
                        eval(parts)
                    }
                };
                current = apply(current, num, op);
                let next = match parts.next() {
                    Some(p) => { p }
                    None => { return current; }
                };
                if next == ')' {
                    return current;
                }
                op = next;
            }
        }
    }
}

fn eval_str(string: &str) -> i64 {
    let mut filtered = string.chars().filter(|c| *c != ' ');
    eval(&mut filtered)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let total: i64 = input.lines().map(|line| eval_str(line)).sum();
    println!("{}", total);
}
