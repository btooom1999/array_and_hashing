const DIRECTIONS: [(i32,i32); 4] = [(0,1),(0,-1),(1,0),(-1,0)];

fn dfs(
    i: i32,
    j: i32,
    m: i32,
    n: i32,
    visited: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<i32>>,
    mut sum: i32,
    max: &mut i32,
) {
    sum += grid[i as usize][j as usize];
    visited[i as usize][j as usize] = true;
    for direct in DIRECTIONS {
        let i = direct.0 + i;
        let j = direct.1 + j;
        if i < 0 || j < 0 || i == m || j == n || visited[i as usize][j as usize] || grid[i as usize][j as usize] == 0 {
            continue;
        }

        dfs(i, j, m, n, visited, grid, sum, max);
    }

    visited[i as usize][j as usize] = false;
    *max = std::cmp::max(*max, sum);
}

fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut max = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != 0 {
                dfs(i as i32, j as i32, m, n, &mut visited, &grid, 0, &mut max);
            }
        }
    }

    max
}

pub fn main() {
    let grid = [[0,6,0],[5,8,7],[0,9,0]].into_iter().map(Vec::from).collect();
    println!("{}", get_maximum_gold(grid));
}
