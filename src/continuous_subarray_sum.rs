use std::collections::HashMap;

fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut hashmap = HashMap::<i32, i32>::from([(0, -1)]);
    let mut res = 0;
    let mut sum = 0;
    for (j, num) in nums.iter().enumerate() {
        sum += *num;
        if let Some(i) = hashmap.get(&(sum % k))
            && j as i32 - *i > 1
        {
            return true;
        }

        hashmap.entry(sum % k).or_insert(j as i32);
    }

    false
}

pub fn main() {
    let nums = vec![23, 2, 6, 4, 7];
    let k = 13;
    println!("{}", check_subarray_sum(nums, k));
}
