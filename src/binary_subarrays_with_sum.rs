use std::collections::HashMap;

fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::from([(0, 1)]);
    let mut sum = 0;
    let mut res = 0;
    for num in &nums {
        sum += num;
        if let Some(val) = hashmap.get(&(sum - goal)) {
            res += *val;
        }

        *hashmap.entry(sum).or_default() += 1;
    }

    res
}

pub fn main() {
    let nums = vec![1, 0, 1, 0, 1];
    let goal = 2;
    println!("{}", num_subarrays_with_sum(nums, goal));
}
