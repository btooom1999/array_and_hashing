use std::collections::VecDeque;

fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut count = 0;
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 2 {
                queue.push_back((i as i32, j as i32, 1));
            } else if cell == 1 {
                count += 1;
            }
        }
    }

    if count == 0 {
        return 0;
    }

    let mut res = 0;
    let directions = [(0,1),(0,-1),(1,0),(-1,0)];
    while let Some((i, j, times)) = queue.pop_front() {
        for direct in directions {
            let i = direct.0 + i;
            let j= direct.1 + j;
            if i < 0 || j < 0 || i == m || j == n || grid[i as usize][j as usize] == 2 || grid[i as usize][j as usize] == 0 {
                continue;
            }

            grid[i as usize][j as usize] = 2;
            count -= 1;
            queue.push_back((i, j, times+1));
        }
        if count == 0 {
            res = times;
            break;
        }
    }

    if count > 0 {
        return -1;
    }

    res
}

pub fn main() {
    let grid = [[2,1,1],[1,1,0],[0,1,1]].into_iter().map(Vec::from).collect();
    println!("{}", oranges_rotting(grid));
}

