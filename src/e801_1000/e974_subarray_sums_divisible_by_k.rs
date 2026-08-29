use std::collections::HashMap;

fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::from([(0, 1)]);
    let mut res = 0;
    let mut sum = 0;
    for num in &nums {
        sum += *num;
        let val = ((sum % k) + k) % k;
        if let Some(val) = hashmap.get(&val.abs()) {
            res += *val;
        }
        *hashmap.entry(val).or_default() += 1;
    }

    res
}

pub fn main() {
    let nums = vec![-1, 2, 9];
    let k = 2;
    println!("{}", subarrays_div_by_k(nums, k));
}
