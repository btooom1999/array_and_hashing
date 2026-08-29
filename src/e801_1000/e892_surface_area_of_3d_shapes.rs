fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] > 0 {
                res += 2 + grid[i][j] * 4;
                if i>0 {
                    res -= 2 * grid[i-1][j].min(grid[i][j]);
                }
                if j>0 {
                    res -= 2 * grid[i][j-1].min(grid[i][j]);
                }
            }
        }
    }

    res
}

pub fn main() {
    let grid = [[1,2],[3,4]].into_iter().map(Vec::from).collect();
    println!("{}", surface_area(grid));
}
