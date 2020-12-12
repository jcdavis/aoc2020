use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn sums_to(prev: &HashMap<i64, i32>, to: i64) -> bool {
    for (num, count) in prev {
        let rest = to-num;
        if (rest == *num && *count > 1) || (rest != *num && prev.contains_key(&rest)) {
            return true;
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let nums: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let running_size: usize = 25;
    let mut prev: HashMap<i64, i32> = HashMap::new();

    for i in 0..running_size {
        *prev.entry(nums[i]).or_insert(0) += 1;
    }

    for i in 0..nums.len() {
        let mut total: i64 = 0;
        let mut j = i;
        loop {
            total += nums[j];
            if total == 104054607 || j == 0 {
                break;
            }
            j -= 1;
        }
        if total == 104054607 {
            println!("Found it: {} to {}", j, i);
            let mut copy: Vec<&i64> = nums[j..i+1].iter().collect();
            copy.sort();
            println!("{}", copy[0] + copy[copy.len() - 1]);
        }
    }
    /*for i in running_size..nums.len() {
        if !sums_to(&prev, nums[i]) {
            println!("{}", &nums[i]);
        }
        *prev.entry(nums[i]).or_default() += 1;

        match prev.entry(nums[i - running_size]) {
            Occupied(mut o) => {
                if *o.get() == 1 {
                    o.remove();
                } else {
                    *o.get_mut() -= 1;
                }
            }
            _ => panic!("wtf")
        }
        // 104054607
    }*/
}
