const DIRECTIONS: [(i32, i32); 4] = [(0,1), (0,-1), (1,0), (-1,0)];

fn dfs(
    i: i32,
    j: i32,
    m: i32,
    n: i32,
    valid: &mut bool,
    grid: &mut Vec<Vec<i32>>,
) -> i32 {
    if i == 0 || j == 0 || i == m-1 || j == n-1 {
        *valid = false;
    }

    grid[i as usize][j as usize] = 0;
    let mut count = 1;
    for direct in DIRECTIONS {
        let i = direct.0 + i;
        let j = direct.1 + j;
        if i < 0 || j < 0 || i == m || j == n || grid[i as usize][j as usize] == 0 {
            continue;
        }

        count += dfs(i, j, m, n, valid, grid);
    }

    count
}

fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut res = 0;
    for i in 1..m-1 {
        for j in 1..n-1 {
            if grid[i as usize][j as usize] == 1 {
                let mut valid = true;
                let count = dfs(i, j, m, n, &mut valid, &mut grid);
                if valid {
                    res += count;
                }
            }
        }
    }

    res
}

pub fn main() {
    let grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]].into_iter().map(Vec::from).collect();
    println!("{:?}", num_enclaves(grid));
}
