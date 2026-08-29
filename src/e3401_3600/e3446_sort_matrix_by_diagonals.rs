fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            if j > i {
                for k in 1..=i {
                    if i >= k && j >= k && grid[i-k][j-k] > grid[i-k+1][j-k+1] {
                        (grid[i-k][j-k], grid[i-k+1][j-k+1]) = (grid [i-k+1][j-k+1], grid[i-k][j-k]);
                    } else {
                        break;
                    }
                }
            } else {
                for k in 1..=i {
                    if i >= k && j >= k && grid[i-k][j-k] < grid[i-k+1][j-k+1] {
                        (grid[i-k][j-k], grid[i-k+1][j-k+1]) = (grid [i-k+1][j-k+1], grid[i-k][j-k]);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    grid
}

pub fn main() {
    let grid = [[3,3,1,1],[2,2,1,2],[1,1,1,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", sort_matrix(grid));
}
