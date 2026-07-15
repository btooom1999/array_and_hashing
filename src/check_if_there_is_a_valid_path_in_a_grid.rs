fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; n]; m];
    visited[0][0] = true;
    let mut queue = std::collections::VecDeque::from([(0,0, grid[0][0])]);
    while let Some((i, j, kind)) = queue.pop_front() {
        if i == m-1 && j == n-1 {
            return true;
        }

        match kind {
            1 => {
                if j+1<n && [1,3,5].contains(&grid[i][j+1]) && !visited[i][j+1] {
                    visited[i][j+1] = true;
                    queue.push_back((i, j+1, grid[i][j+1]));
                }
                if j>0 && [1,4,6].contains(&grid[i][j-1]) && !visited[i][j-1] {
                    visited[i][j-1] = true;
                    queue.push_back((i, j-1, grid[i][j-1]));
                }
            }
            2 => {
                if i+1<m && [2,5,6].contains(&grid[i+1][j]) && !visited[i+1][j] {
                    visited[i+1][j] = true;
                    queue.push_back((i+1, j, grid[i+1][j]));
                }
                if i>0 && [2,3,4].contains(&grid[i-1][j]) && !visited[i-1][j] {
                    visited[i-1][j] = true;
                    queue.push_back((i-1, j, grid[i-1][j]));
                }
            }
            3 => {
                if i+1<m && [2,5,6].contains(&grid[i+1][j]) && !visited[i+1][j] {
                    visited[i+1][j] = true;
                    queue.push_back((i+1, j, grid[i+1][j]));
                }
                if j>0 && [1,4,6].contains(&grid[i][j-1]) && !visited[i][j-1] {
                    visited[i][j-1] = true;
                    queue.push_back((i, j-1, grid[i][j-1]));
                }
            }
            4 => {
                if j+1<n && [1,3,5].contains(&grid[i][j+1]) && !visited[i][j+1] {
                    visited[i][j+1] = true;
                    queue.push_back((i, j+1, grid[i][j+1]));
                }
                if i+1<m && [2,5,6].contains(&grid[i+1][j]) && !visited[i+1][j] {
                    visited[i+1][j] = true;
                    queue.push_back((i+1, j, grid[i+1][j]));
                }
            }
            5 => {
                if i>0 && [2,3,4].contains(&grid[i-1][j]) && !visited[i-1][j] {
                    visited[i-1][j] = true;
                    queue.push_back((i-1, j, grid[i-1][j]));
                }
                if j>0 && [1,4,6].contains(&grid[i][j-1]) && !visited[i][j-1] {
                    visited[i][j-1] = true;
                    queue.push_back((i, j-1, grid[i][j-1]));
                }
            }
            6 => {
                if i>0 && [2,3,4].contains(&grid[i-1][j]) && !visited[i-1][j] {
                    visited[i-1][j] = true;
                    queue.push_back((i-1, j, grid[i-1][j]));
                }
                if j+1<n && [1,3,5].contains(&grid[i][j+1]) && !visited[i][j+1] {
                    visited[i][j+1] = true;
                    queue.push_back((i, j+1, grid[i][j+1]));
                }
            }
            _ => unreachable!()
        }
    }

    false
}

pub fn main() {
    let grid = [[2,4,3],[6,5,2]].into_iter().map(Vec::from).collect();
    println!("{}", has_valid_path(grid));
}
