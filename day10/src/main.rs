use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut nums: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    nums.sort();

    let mut diff_1 = 0;
    let mut diff_3 = 1;
    let mut last = 0;
    for num in &nums {
        if num - last == 1 {
            diff_1 += 1;
        }
        if num - last == 3 {
            diff_3 += 1;
        }
        last = *num;
    }
    println!("{}", diff_1*diff_3);
}
