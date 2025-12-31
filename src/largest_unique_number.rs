use std::collections::HashMap;

fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::with_capacity(nums.len());
    for num in nums {
        *hashmap.entry(num).or_insert(0) += 1;
    }

    let mut max = -1;
    for (k, v) in hashmap.into_iter() {
        if v == 1 {
            max = std::cmp::max(max, k);
        }
    }

    max
}

pub fn main() {
    let nums = vec![5, 7, 3, 9, 4, 9, 8, 3, 1];
    println!("{}", largest_unique_number(nums));
}
