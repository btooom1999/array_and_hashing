use std::collections::HashMap;

fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::<i32, bool>::new();
    for &num in &nums {
        hashmap.entry(num).and_modify(|v| *v = true).or_default();
    }

    hashmap.iter().filter_map(|(&k, v)| (
        !v
        && !hashmap.contains_key(&(k+1))
        && !hashmap.contains_key(&(k-1))
    ).then_some(k)).collect()
}

pub fn main() {
    let nums = [10,10,6,5,8].to_vec();
    println!("{:?}", find_lonely(nums));
}
