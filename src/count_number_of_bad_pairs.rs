use std::collections::HashMap;

pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let n = nums.len() as i64;
    let mut hashmap = HashMap::<i32, i64>::new();
    let mut res = 0i64;
    for (i, num) in nums.iter().enumerate() {
        let val = hashmap.entry(num - i as i32).or_default();
        res += *val;
        *val += 1;
    }

    ((n - 1) * n / 2) - res
}

pub fn main() {
    let nums = vec![4,1,3,3];
    println!("{}", count_bad_pairs(nums));
}
