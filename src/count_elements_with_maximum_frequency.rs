use std::collections::HashMap;

fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::new();
    let mut max = 0;

    for &num in &nums {
        let val = hashmap.entry(num).or_default();
        *val += 1;

        max = max.max(*val);
    }

    hashmap.into_iter().fold(0, |acc, (k, v)| acc + (if max == v { v } else { 0 }))
}

pub fn main() {
    let nums = [1,2,2,3,1,4].to_vec();
    println!("{}", max_frequency_elements(nums));
}
