use std::collections::HashSet;

fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    HashSet::from_iter(1..=nums.len() as i32)
        .difference(&HashSet::<i32>::from_iter(nums))
        .cloned()
        .collect()
}

pub fn main() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("{:?}", find_disappeared_numbers(nums));
}
