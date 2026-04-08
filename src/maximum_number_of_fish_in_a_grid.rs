const DIRECTIONS: [(i32, i32); 4] = [(-1,0), (1,0), (0, -1), (0, 1)];

fn dfs(
    m: i32,
    n: i32,
    i: i32,
    j: i32,
    grid: &mut Vec<Vec<i32>>,
) -> i32 {

    let mut value = grid[i as usize][j as usize];
    grid[i as usize][j as usize] = 0;
    for direct in DIRECTIONS {
        let i = direct.0 + i;
        let j = direct.1 + j;
        if i < 0 || j < 0 || i == m || j == n || grid[i as usize][j as usize] == 0 {
            continue;
        }

        value += dfs(m, n, i, j, grid);
    }

    value
}

fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] > 0 {
                count = count.max(dfs(m, n, i, j, &mut grid))
            }
        }
    }

    count
}

pub fn main() {
    let grid = [[0,2,1,0],[4,0,0,3],[1,0,0,4],[0,3,2,0]].into_iter().map(Vec::from).collect();
    println!("{}", find_max_fish(grid));
}
