fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut res = 0;
    for i in 0..nums.len() {
        sum += nums[i] as i64;
        res = res.max((sum as f64 / (i+1) as f64).ceil() as i32);
    }

    res
}

pub fn main() {
    let nums = [3,7,1,6].to_vec();
    println!("{}", minimize_array_value(nums));
}
