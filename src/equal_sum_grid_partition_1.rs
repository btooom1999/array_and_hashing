fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    let mut total = 0;
    let mut grid = grid
        .into_iter()
        .map(|v| v
            .into_iter()
            .map(|num| {
                total += num as i64;
                num as i64
            }).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..m {
        for j in 0..n {
            if j > 0 {
                grid[i][j] += grid[i][j-1];
            }
            if i > 0 {
                grid[i][j] += grid[i-1][j];
            }
            if i > 0 && j > 0 {
                grid[i][j] -= grid[i-1][j-1];
            }
            if (j == n-1 || i == m-1) && total - grid[i][j] == grid[i][j] {
                return true;
            }
        }
    }

    false
}

// [50042, 90066, 147758, 162041]
// [85105, 225129, 382821, 450258]

pub fn main() {
    // let grid = [[1,2],[4,3]].into_iter().map(Vec::from).collect();
    let grid = [
        [50042,40024,57692,14283],
        [35063,100000,100000,53154]
    ].into_iter().map(Vec::from).collect();
    println!("{}", can_partition_grid(grid));
}
