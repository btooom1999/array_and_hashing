fn max_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort();

    let mut res = i32::MIN;
    let mut sum = 0;
    let mut num = i32::MIN;
    for i in (0..nums.len()).rev() {
        if nums[i] != num {
            sum += nums[i];
            num = nums[i];
            res = res.max(sum);
        }
    }

    res
}

pub fn main() {
    let nums = [1,1,0,1,1].to_vec();
    println!("{}", max_sum(nums));
}
