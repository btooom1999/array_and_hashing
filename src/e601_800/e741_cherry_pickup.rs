use std::cmp::{max, min};

fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut dp = vec![vec![i32::MIN; n]; n];
    dp[0][0] = grid[0][0];

    for t in 1..=(2*n-2) {
        for r1 in (max(0, t.saturating_sub(n-1))..=min(n-1, t)).rev() {
            for r2 in (max(0, t.saturating_sub(n-1))..=min(n-1, t)).rev() {
                let c1 = t - r1;
                let c2 = t - r2;

                let mut max = i32::MIN;
                if grid[r1][c1] != -1 && grid[r2][c2] != -1 {
                    let mut cherry = grid[r1][c1];
                    if r1 != r2 { cherry += grid[r2][c2]; }

                    max = max.max(dp[r1][r2]);
                    if r1>0 { max = max.max(dp[r1-1][r2]); }
                    if r2>0 { max = max.max(dp[r1][r2-1]); }
                    if r1>0 && r2>0 { max = max.max(dp[r1-1][r2-1]); }

                    if max != i32::MIN { max += cherry };
                }

                dp[r1][r2] = max;
            }
        }
    }

    dp[n-1][n-1].max(0)
}

pub fn main() {
    let grid = [[0,1,-1],[1,0,-1],[1,1,1]].into_iter().map(Vec::from).collect();
    println!("{}", cherry_pickup(grid));
}
