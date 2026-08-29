fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![(0, 1, 1); n+1]; m];
    let mut res = 0;
    for i in 0..m {
        for j in (0..n).rev() {
            if grid[i][j] == 1 {
                dp[i][j].0 = 1 + dp[i][j+1].0;
            }

            if i>0 && j+1<n && dp[i][j].0 >= 3 {
                let mut count = dp[i][j].0;
                if count % 2 == 0 { count -= 1; };
                if grid[i-1][j+1] == 1 {
                    if dp[i-1][j+1].1 >= count-2 {
                        dp[i][j].1 = count;
                    } else {
                        dp[i][j].1 = dp[i-1][j+1].1+2;
                    }
                }
            }

            res += (dp[i][j].1 - 3) / 2 + 1;
        }
    }

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i+1<m && j+1<n && dp[i][j].0 >= 3 {
                let mut count = dp[i][j].0;
                if count % 2 == 0 { count -= 1; };
                if grid[i+1][j+1] == 1 {
                    if dp[i+1][j+1].2 >= count-2 {
                        dp[i][j].2 = count;
                    } else {
                        dp[i][j].2 = dp[i+1][j+1].2+2;
                    }
                }
            }

            res += (dp[i][j].2 - 3) / 2 + 1;
        }
    }

    res
}

pub fn main() {
    // let grid = [[1,1,1,1,0],[1,1,1,1,1],[1,1,1,1,1],[0,1,0,0,1]].into_iter().map(Vec::from).collect();
    let grid = [[0,0,0,0,1,0,1,0,0,0],[0,0,0,1,1,1,1,1,0,0],[0,0,1,1,1,1,1,1,1,0],[0,1,1,1,1,1,1,1,1,0],[1,1,1,1,1,1,1,1,0,1],[1,1,1,1,1,1,1,1,1,1]].into_iter().map(Vec::from).collect();
    println!("{}", count_pyramids(grid));
}

