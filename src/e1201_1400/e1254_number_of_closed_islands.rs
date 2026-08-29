const DIRECTIONS: [(i32, i32); 4] = [(0,1), (0,-1), (1,0), (-1,0)];

fn dfs(
    i: i32,
    j: i32,
    m: i32,
    n: i32,
    grid: &mut Vec<Vec<i32>>,
    valid: &mut bool,
) {
    grid[i as usize][j as usize] = 1;
    if i == 0 || i == m-1 || j == 0 || j == n-1 {
        *valid = false;
    }

    for direct in DIRECTIONS {
        let i = direct.0 + i;
        let j = direct.1 + j;
        if i < 0 || j < 0 || i == m || j == n || grid[i as usize][j as usize] == 1 {
            continue;
        }
        dfs(i, j, m, n, grid, valid);
    }
}
fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut count = 0;
    for i in 1..m-1 {
        for j in 1..n-1 {
            if grid[i][j] == 0 {
                let mut valid = true;
                dfs(i as i32, j as i32, m as i32, n as i32, &mut grid, &mut valid);
                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn main() {
    let grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]].into_iter().map(Vec::from).collect();
    println!("{:?}", closed_island(grid))
}
