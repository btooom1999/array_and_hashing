use std::collections::HashMap;

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::with_capacity(nums.len());

    for num in &nums {
        hashmap.entry(*num).and_modify(|v| *v += 1).or_insert(1);
    }

    for (k, v) in hashmap.iter() {
        if *v > nums.len() / 2 {
            return *k;
        }
    }

    0
}

pub fn main() {
    let nums = vec![3, 2, 3];

    println!("{}", majority_element(nums));
}
