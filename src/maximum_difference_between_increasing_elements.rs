fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut min = nums[0];
    let mut res = -1;

    for i in 1..nums.len() {
        if nums[i]<=min {
            min = nums[i];
        } else {
            res = res.max(nums[i]-min);
        }
    }

    res
}

pub fn main() {
    let nums = [7,1,5,4].to_vec();
    println!("{}", maximum_difference(nums));
}
