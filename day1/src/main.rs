use std::env;
use std::fs;

use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let nums: HashSet<i32> = input.lines().map(|line| line.parse::<i32>().unwrap()).collect();

    for num1 in &nums {
        for num2 in &nums {
            let rem = 2020-num1-num2;
            if nums.contains(&rem) {
                println!("{}", num1*num2*rem);
            }
        }
    }
}
