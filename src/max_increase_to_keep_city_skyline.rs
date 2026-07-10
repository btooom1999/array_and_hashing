fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut max_rows = vec![0; m];
    let mut max_cols = vec![0; n];

    for i in 0..m {
        for j in 0..n {
            max_rows[i] = max_rows[i].max(grid[i][j]);
            max_cols[j] = max_cols[j].max(grid[i][j]);
        }
    }

    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            res += max_rows[i].min(max_cols[j]) - grid[i][j];
        }
    }

    res
}

pub fn main() {
    let grid = [[3,0,8,4],[2,4,5,7],[9,2,6,3],[0,3,1,0]].into_iter().map(Vec::from).collect();
    println!("{}", max_increase_keeping_skyline(grid));
}
