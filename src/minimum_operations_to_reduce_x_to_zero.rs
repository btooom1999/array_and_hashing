use std::collections::HashMap;

fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let target = nums.iter().sum::<i32>() - x;
    let mut sum = 0;
    let mut res = if sum == target {
        nums.len() as i32
    } else {
        i32::MAX
    };
    let mut l = 0;
    for (r, num) in nums.iter().enumerate() {
        sum += *num;
        while sum >= target && l <= r {
            if sum == target {
                res = res.min((nums.len() - (r - l + 1)) as i32);
            }
            sum -= nums[l];
            l += 1;
        }
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let nums = vec![1, 2, 3, 4];
    let x = 10;
    println!("{}", min_operations(nums, x));
}
