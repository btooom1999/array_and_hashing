fn max_points(points: Vec<Vec<i32>>) -> i64 {
    let (m, n) = (points.len(), points[0].len());
    let mut dp = vec![vec![(0, 0); n]; m];
    dp[0] = (0..n).map(|j| (points[0][j] as i64, j)).collect();

    for i in 1..m {
        for j in 0..n {
            let val = points[i][j] as i64;
            dp[i][j] = (val + dp[i-1][j].0, j);
            if j>0 && val + dp[i-1][dp[i][j-1].1].0 - (j as i64 - dp[i][j-1].1 as i64).abs() > dp[i][j].0 {
                dp[i][j].0 = val + dp[i-1][dp[i][j-1].1].0 - (j as i64 - dp[i][j-1].1 as i64).abs();
                dp[i][j].1 = dp[i][j-1].1;
            }
        }

        for j in (0..n).rev() {
            let val = points[i][j] as i64;
            if j+1<n && val + dp[i-1][dp[i][j+1].1].0 - (j as i64 - dp[i][j+1].1 as i64).abs() > dp[i][j].0 {
                dp[i][j].0 = val + dp[i-1][dp[i][j+1].1].0 - (j as i64 - dp[i][j+1].1 as i64).abs();
                dp[i][j].1 = dp[i][j+1].1;
            }
        }
    }

    dp[m-1].iter().max_by_key(|v| v.0).unwrap().0
}

pub fn main() {
    let points = [[1,2,3],[1,5,1],[3,1,1]].into_iter().map(Vec::from).collect();
    // let points = [[5,2,1,2],[2,1,5,2],[5,5,5,0]].into_iter().map(Vec::from).collect();
    println!("{}", max_points(points));
}
