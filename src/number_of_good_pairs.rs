use std::collections::HashMap;

fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::with_capacity(nums.len());
    for val in &nums {
        *hashmap.entry(*val).or_default() += 1;
    }

    let mut count = 0;
    for val in &nums {
        if let Some(num) = hashmap.get_mut(val)
            && *num > 1
        {
            *num -= 1;
            count += *num;
        }
    }

    count
}

pub fn main() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    println!("{}", num_identical_pairs(nums));
}
