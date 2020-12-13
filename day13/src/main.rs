use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let start: i32 = lines[0].parse().unwrap();
    let nums: Vec<i32> = lines[1].split(",").filter(|x| *x != "x").map(|n| {
        n.parse().unwrap()
    }).collect();

    let mut best_bus = -1;
    let mut best_time = 1000000000;

    for num in nums {
        let times = start/num;
        let diff = num*(times+1) - start;
        if diff < best_time {
            best_time = diff;
            best_bus = num;
        }
    }
    println!("{}", best_bus*best_time);
}
