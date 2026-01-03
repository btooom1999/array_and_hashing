use std::collections::HashMap;

fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::from([(0, 1)]);
    let mut odd_count = 0;
    let mut res = 0;
    for num in &nums {
        if num % 2 != 0 {
            odd_count += 1;
        }

        if let Some(val) = hashmap.get(&(odd_count - k)) {
            res += *val;
        }

        *hashmap.entry(odd_count).or_default() += 1;
    }

    res
}

pub fn main() {
    let nums = vec![1, 1, 2, 1, 1];
    // let nums = vec![2, 4, 6];
    // let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];

    let k = 3;
    println!("{}", number_of_subarrays(nums, k));
}
