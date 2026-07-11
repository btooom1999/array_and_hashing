fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut res = vec![vec![0; n]; m];
    let mut k = m * n - (k as usize % (m * n));

    for i in 0..m {
        for j in 0..n {
            if k == m*n { k = 0; }
            res[i][j] = grid[k/n][k%n];
            k += 1;
        }
    }

    res
}

pub fn main() {
    let grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]].into_iter().map(Vec::from).collect();
    let k = 4;
    println!("{:?}", shift_grid(grid, k));
}
