use std::collections::VecDeque;

const DIRECTION: [(i32, i32); 8] = [(1,1), (-1,-1), (-1,1), (1,-1), (0,1), (0,-1), (1,0), (-1,0)];

fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if grid[0][0] != 0 || grid[n-1][n-1] != 0 {
        return -1;
    }

    let n = n as i32;
    let mut queue = VecDeque::from([(0,0,1)]);
    grid[0][0] = 1;

    while let Some((i, j, dist)) = queue.pop_front() {
        if i == n-1 && j == n-1 {
            return dist;
        }

        for direct in DIRECTION {
            let i = direct.0 + i;
            let j = direct.1 + j;
            if i < 0 || j < 0 || i == n || j == n || grid[i as usize][j as usize] == 1 {
                continue;
            }


            grid[i as usize][j as usize] = 1;
            queue.push_back((i, j, dist+1));
        }
    }

    -1
}

pub fn main() {
    // let grid = [[0,1],[1,0]].into_iter().map(Vec::from).collect();
    let grid = [[0,0,1,0,1],[0,0,0,1,0],[1,0,0,1,1],[0,0,0,1,1],[1,0,0,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", shortest_path_binary_matrix(grid));
}
