fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut res = 0;
    for i in 0..n {
        let mut max_col = 0;
        let mut max_row = 0;
        for j in 0..n {
            res += (grid[i][j] > 0) as i32;
            max_col = max_col.max(grid[i][j]);
            max_row = max_row.max(grid[j][i]);
        }

        res += max_col + max_row;
    }

    res
}

pub fn main() {
    let grid = [[1,2],[3,4]].into_iter().map(Vec::from).collect();
    println!("{}", projection_area(grid));
}
