use std::collections::HashMap;

fn main() {
    let nums = vec![1,20,8,12,0,14];
    let mut last_seen: HashMap<i32, usize> = HashMap::new();
    let mut i: usize = 0;
    let mut next_num = 0;
    while i < nums.len() {
        last_seen.insert(nums[i], i);
        i += 1;
    }
    while i < 30000000-1 {
        if !last_seen.contains_key(&next_num) {
            last_seen.insert(next_num, i);
            next_num = 0;
        } else {
            let prev_pos = last_seen[&next_num];
            last_seen.insert(next_num, i);
            next_num = (i - prev_pos) as i32;
        }
        i += 1;
    }
    println!("{}", next_num);
}
