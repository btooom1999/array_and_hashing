use std::collections::HashMap;

fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut sum = 0;
    let mut hashmap = HashMap::new();
    let mut res = i64::MIN;
    for i in 0..n {
        let num = nums[i] as i64;
        for k in [nums[i]-k, nums[i]+k] {
            if let Some(&start_sum) = hashmap.get(&k) {
                res = res.max(sum + num - start_sum);
            }
        }
        hashmap.entry(nums[i]).and_modify(|v| if *v > sum { *v = sum }).or_insert(sum);
        sum += nums[i] as i64;
    }

    if res == i64::MIN { 0 } else { res }
}

pub fn main() {
    let nums = [2,3,3].to_vec();
    let k = 1;
    println!("{}", maximum_subarray_sum(nums, k));
}
