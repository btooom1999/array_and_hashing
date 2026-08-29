fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    let mut dp = vec![0; nums.len()];
    let mut res = 0;

    for i in 0..nums.len() {
        if i > 0 {
            dp[i] = dp[i >> 1] + (i as i32 & 1);
        }

        if dp[i] == k {
            res += nums[i];
        }
    }

    res
}

pub fn main() {
    let nums = [5,10,1,5,2].to_vec();
    let k = 1;
    println!("{}", sum_indices_with_k_set_bits(nums, k))
}
