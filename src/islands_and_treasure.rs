use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(-1,0), (1,0), (0, -1), (0, 1)];

fn dfs(
    m: i32,
    n: i32,
    i: i32,
    j: i32,
    grid: &mut Vec<Vec<i32>>,
    moved: &mut HashSet<(i32, i32)>,
) -> i32 {
    let mut positions = Vec::new();
    let mut step = i32::MAX;

    for direct in DIRECTIONS {
        let ni = direct.0 + i;
        let nj = direct.1 + j;
        if ni < 0 || nj < 0 || ni == m || nj == n || moved.contains(&(ni, nj)) {
            continue;
        }

        let val = grid[ni as usize][nj as usize];
        if val == -1 {
            continue;
        }

        if val == 0 {
            grid[i as usize][j as usize] = 1;
            return 1;
        }

        if val == i32::MAX {
            positions.push((ni, nj));
        } else {
            step = step.min(1 + val);
        }
    }

    moved.insert((i, j));
    for (i, j) in positions {
        let val = dfs(m, n, i, j, grid, moved);
        if val != i32::MAX {
            step = step.min(1 + val);
        }
    }
    moved.remove(&(i, j));

    grid[i as usize][j as usize] = step;
    step
}

fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut hashset = HashSet::new();
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] > 0 {
                dfs(m, n, i, j, grid, &mut hashset);
            }
        }
    }
}

pub fn main() {
    let mut grid = [
        [2147483647,-1,        0         ,2147483647].to_vec(),
        [2147483647,2147483647,2147483647,-1].to_vec(),
        [2147483647,-1,        2147483647,-1].to_vec(),
        [0,         -1,        2147483647,2147483647].to_vec()
    ].to_vec();
    // let mut grid = [
    //     [2147483647,2147483647,2147483647].to_vec(),
    //     [2147483647,-1,2147483647].to_vec(),
    //     [0,2147483647,2147483647].to_vec()
    // ].to_vec();
    islands_and_treasure(&mut grid);
    println!("{:?}", grid);
}
