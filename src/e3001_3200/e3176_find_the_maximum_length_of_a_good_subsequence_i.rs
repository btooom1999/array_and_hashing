fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let (n, k) = (nums.len(), k as usize);
    let mut dp = vec![vec![1; k+1]; n];
    let mut res = 1;

    for i in 0..n {
        for j in 0..k+1 {
            for prev in 0..i {
                if nums[i] == nums[prev] {
                    dp[i][j] = dp[i][j].max(dp[prev][j]+1);
                } else if j > 0 {
                    dp[i][j] = dp[i][j].max(dp[prev][j-1]+1);
                }

                res = res.max(dp[i][j]);
            }
        }
    }

    res
}

pub fn main() {
    let nums = vec![1,2,1,1,3];
    let k = 2;
    println!("{}", maximum_length(nums, k));
}
