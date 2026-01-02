use std::collections::HashMap;

fn subarray_sum(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::from([(0, 1)]);
    let mut res = 0;
    let mut sum = 0;
    for num in &nums {
        sum += num;
        if let Some(val) = hashmap.get(&(sum - k)) {
            res += *val;
        }
        *hashmap.entry(sum).or_default() += 1;
    }

    res
}

pub fn main() {
    let nums = vec![1, 2, 1, 2, 1];
    let k = 3;
    println!("{}", subarray_sum(nums, k));
}
