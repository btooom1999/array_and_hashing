fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut prefix = vec![vec![0; n+1]; m+1];

    for i in 0..m {
        for j in 0..n {
            prefix[i+1][j+1] = prefix[i+1][j];
            prefix[i+1][j+1] += grid[i][j];
        }
    }

    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            prefix[i+1][j+1] += prefix[i][j+1];
            if prefix[i+1][j+1] <= k {
                res += 1;
            }
        }
    }

    res
}

pub fn main() {
    let grid = [[7,6,3],[6,6,1]].into_iter().map(Vec::from).collect::<Vec<_>>();
    let k = 18;
    println!("{}", count_submatrices(grid, k));
}
