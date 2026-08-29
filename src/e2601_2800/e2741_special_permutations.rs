const MOD: i32 = 1_000_000_007;

fn dfs(
    nums: &[i32],
    visited: &mut usize,
    last: usize,
    memo: &mut Vec<Vec<i32>>,
) -> i32 {
    if *visited == (1 << nums.len())-1 {
        return 1;
    }

    if last != usize::MAX && memo[*visited][last] != -1 {
        return memo[*visited][last];
    }

    let mut count = 0;
    for i in 0..nums.len() {
        if last == usize::MAX || (*visited & 1 << i == 0 && (nums[i] % nums[last] == 0 || nums[last] % nums[i] == 0)) {
            *visited ^= 1 << i;
            count = (count + dfs(nums, visited, i, memo)) % MOD;
            *visited ^= 1 << i;
        }
    }

    if last != usize::MAX {
        memo[*visited][last] = count;
    }
    count
}

fn special_perm(nums: Vec<i32>) -> i32 {
    dfs(&nums, &mut 0, usize::MAX, &mut vec![vec![-1; nums.len()]; 1 << nums.len()])
}

pub fn main() {
    let nums = [2,3,6].to_vec();
    println!("{}", special_perm(nums));
}
