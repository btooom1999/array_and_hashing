fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in (0..nums.len()-1).rev() {
        if nums[i] == 1 || nums[i] <= nums[i+1] {
            continue;
        }

        if let Some(factor) = (2..=nums[i].isqrt()).find(|&num| nums[i] % num == 0) && factor <= nums[i+1] {
            nums[i] = factor;
            res += 1;
        } else {
            return -1;
        }
    }

    res
}

pub fn main() {
    let nums = [9,2].to_vec();
    println!("{}", min_operations(nums));
}
