use std::collections::{HashMap, HashSet};

fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::new();
    for num in &nums {
        *hashmap.entry(*num).or_default() += 1;
    }

    let mut res = 0;
    let mut hashset = HashSet::<i32>::new();
    for (key, count) in hashmap.iter() {
        let need1 = key - k;
        let need2 = key + k;

        if !hashset.contains(&need1) && let Some(count1) = hashmap.get(&need1) {
            res += count * count1;
        }

        if !hashset.contains(&need2) && let Some(count2) = hashmap.get(&need2) {
            res += count * count2;
        }

        hashset.insert(*key);
    }

    res
}

pub fn main() {
    let nums = vec![1,2,2,1];
    let k = 1;
    println!("{}", count_k_difference(nums, k));
}
