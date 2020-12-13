use std::env;
use std::fs;

fn first_match(cur_offset: i64, cur_interval: i64, new_offset: i64, new_interval: i64, diff: i64) -> i64 {
    let mut left: i64 = cur_offset;
    let mut right: i64 = new_offset;

    while left + diff != right {
        if left < right {
            left += cur_interval;
        } else {
            //optimization otherwise this takes forever
            let times = (left + diff - right) as f64/(new_interval as f64);
            right += new_interval*(times.ceil() as i64);
        }
    }
    return right;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let _start: i32 = lines[0].parse().unwrap();
    let nums: Vec<i64> = lines[1].split(",").map(|n| {
        match n {
            "x" => { 0 }
            r => { r.parse().unwrap() }
        }
    }).collect();

    let mut cur_interval = nums[0];
    let mut cur_offset = 0;
    let mut cur_index = 0;

    for i in 1..nums.len() {
        if nums[i] > 0 {
            cur_offset = first_match(cur_offset, cur_interval, 0, nums[i], i as i64 - cur_index);
            cur_index = i as i64;
            cur_interval *= nums[i];
            println!("Now offset {} interval {}", cur_offset, cur_interval);
        }
    }
    println!("{}", cur_offset - (nums.len() as i64 - 1));
}
