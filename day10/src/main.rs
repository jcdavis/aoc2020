use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let mut nums: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    nums.push(0); //hack
    nums.sort();

    let mut cache = vec![0 as i64; nums.len()];
    cache[0] = 1;

    for i in 1..nums.len() {
        let n = nums[i];
        let mut prev = 0;
        if i > 0 && n - nums[i-1] <= 3 {
            prev += cache[i-1];
        }
        if i > 1 && n - nums[i-2] <= 3 {
            prev += cache[i-2];
        }
        if i > 2 && n - nums[i-3] <= 3 {
            prev += cache[i-3];
        }
        cache[i] = prev;
    }
    println!("{}", cache[cache.len() - 1]);
}
