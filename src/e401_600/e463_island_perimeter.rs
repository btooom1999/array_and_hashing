fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                continue;
            }

            let mut perimeter = 4;
            if i+1 < m && grid[i+1][j] == 1 {
                perimeter -= 1;
            }
            if i > 0 && grid[i-1][j] == 1 {
                perimeter -= 1;
            }
            if j > 0 && grid[i][j-1] == 1 {
                perimeter -= 1;
            }
            if j+1 < n && grid[i][j+1] == 1 {
                perimeter -= 1;
            }

            sum += perimeter;
        }
    }

    sum
}

pub fn main() {
    let grid = [[1,1]].into_iter().map(Vec::from).collect();
    // let grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", island_perimeter(grid));
}
