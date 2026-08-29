const MOD: i32 = 1_000_000_007;

fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let (k, m, n) = (k as usize, grid.len(), grid[0].len());
    let mut dp = vec![vec![vec![0; k]; n]; m];
    dp[0][0][grid[0][0] as usize % k] = 1;

    for i in 0..m {
        for j in 0..n {
            let exceed = grid[i][j] as usize % k;
            for extra in 0..k {
                let at = (exceed + extra) % k;
                if i>0 {
                    dp[i][j][at] = (dp[i][j][at] + dp[i-1][j][extra]) % MOD;
                }
                if j>0 {
                    dp[i][j][at] = (dp[i][j][at] + dp[i][j-1][extra]) % MOD;
                }
            }
        }
    }

    dp[m-1][n-1][0]
}

pub fn main() {
    let grid = [[5,2,4],[3,0,5],[0,7,2]].into_iter().map(Vec::from).collect();
    // let grid = vec![vec![1,5,3,7,3,2,3,5]];
    let k = 3;
    println!("{}", number_of_paths(grid, k))
}
