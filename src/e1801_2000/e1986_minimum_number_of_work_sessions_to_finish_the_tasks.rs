fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
    let n = tasks.len();
    let full_mask = (1 << n) - 1;

    let mut subset_sum = vec![0; 1 << n];
    for mask in 0..1<<n {
        let mut sum = 0;
        for i in 0..n {
            if mask & (1 << i) != 0 {
                sum += tasks[i];
            }
        }
        subset_sum[mask] = sum;
    }

    let mut dp = vec![i32::MAX; 1<<n];
    dp[0] = 0;

    for mask in 1..1<<n {
        let mut submask = mask;
        while submask > 0 {
            if subset_sum[submask] <= session_time {
                let prev = mask ^ submask;
                if dp[prev] != i32::MAX {
                    dp[mask] = dp[mask].min(dp[prev]+1);
                }
            }
            submask = (submask - 1) & mask;
        }
    }

    dp[full_mask]
}

pub fn main() {
    let tasks = [9,8,8,4].to_vec();
    let session_time = 14;
    println!("{}", min_sessions(tasks, session_time));
}
