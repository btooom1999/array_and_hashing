fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut hashset = [false; 10_001];
    let mut sum = 0;
    let mut res = 0;
    let mut i = 0;
    for j in 0..nums.len() {
        sum += nums[j];
        if hashset[(nums[j]) as usize] {
            while nums[i] != nums[j] {
                sum -= nums[i];
                hashset[(nums[i]) as usize] = false;
                i += 1;
            }

            sum -= nums[i];
            i += 1;
        }

        hashset[(nums[j]) as usize] = true;
        res = res.max(sum);
    }

    res
}

pub fn main() {
    let nums = [4,2,4,5,6].to_vec();
    println!("{}", maximum_unique_subarray(nums));
}
