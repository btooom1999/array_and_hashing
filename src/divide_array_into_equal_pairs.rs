use std::collections::HashMap;

fn divide_array(nums: Vec<i32>) -> bool {
    let mut hashmap = HashMap::<i32, i32>::with_capacity(nums.len());

    for val in &nums {
        *hashmap.entry(*val).or_default() += 1;
    }

    hashmap.values().all(|v| *v % 2 == 0)
}

pub fn main() {
    let nums = vec![3, 2, 3, 2, 2, 2];
    println!("{}", divide_array(nums));
}
