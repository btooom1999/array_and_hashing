use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len() as i32;
    let mut queue = VecDeque::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 1 {
                queue.push_back((i as i32, j as i32, 0));
            }
        }
    }

    let mut max = 0;
    while let Some((i,j,dist)) = queue.pop_front() {
        max = max.max(dist);
        for direct in DIRECTIONS {
            let i = direct.0 + i;
            let j = direct.1 + j;
            if i < 0 || j < 0 || i == n || j == n || grid[i as usize][j as usize] == 1 {
                continue;
            }

            grid[i as usize][j as usize] = 1;
            queue.push_back((i, j, dist+1));
        }
    }

    if max == 0 { -1 } else { max }
}

pub fn main() {
    let grid = [[1,0,1],[0,0,0],[1,0,1]].into_iter().map(Vec::from).collect();
    println!("{}", max_distance(grid));
}
