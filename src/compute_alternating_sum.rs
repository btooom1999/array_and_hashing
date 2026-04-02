fn alternating_sum(nums: Vec<i32>) -> i32 {
    let (mut even, mut odd) = (0, 0);
    for i in 0..nums.len() {
        if i % 2 == 0 {
            even += nums[i];
        } else {
            odd += nums[i];
        }
    }

    even - odd
}

pub fn main() {
    let nums = [1,3,5,7];
    println!("{}", alternating_sum(nums.into()));
}
