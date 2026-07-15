const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let condition = grid[row as usize][col as usize];

    let mut res = grid.clone();
    let mut visited = vec![vec![false; n]; m];
    visited[row as usize][col as usize] = true;
    let mut queue = std::collections::VecDeque::from([(row, col)]);
    while let Some((i, j)) = queue.pop_front() {
        for direct in DIRECTIONS {
            let ni = i + direct.0;
            let nj = j + direct.1;

            if ni < 0 || nj < 0 || ni == m as i32 || nj == n as i32 || grid[ni as usize][nj as usize] != condition {
                res[i as usize][j as usize] = color;
            } else if !visited[ni as usize][nj as usize] {
                visited[ni as usize][nj as usize] = true;
                queue.push_back((ni, nj));
            }
        }
    }

    res
}

pub fn main() {
    let grid = [[1,1],[1,2]].into_iter().map(Vec::from).collect();
    let row = 0;
    let col = 0;
    let color = 3;
    println!("{:?}", color_border(grid, row, col, color));
}
