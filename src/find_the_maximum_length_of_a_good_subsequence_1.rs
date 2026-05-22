fn dfs(i: usize, k: usize, nums: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
    if memo[i][k] != -1 {
        return memo[i][k];
    }

    let mut best = 1;
    for idx in i+1..nums.len() {
        if nums[i] == nums[idx] {
            best = best.max(1+dfs(idx, k, nums, memo));
        } else if k > 0 {
            best = best.max(1+dfs(idx, k-1, nums, memo));
        }
    }

    memo[i][k] = best;
    best
}

fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut res = 0;
    for i in 0..nums.len() {
        res = res.max(dfs(i, k, &nums, &mut vec![vec![-1; k+1]; nums.len()]))
    }

    res
}

pub fn main() {
    let nums = vec![1; 500];
    let k = 0;
    println!("{}", maximum_length(nums, k));
}
