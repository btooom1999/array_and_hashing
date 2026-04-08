const DIRECTIONS: [(i32, i32); 4] = [(-1,0), (1,0), (0, -1), (0, 1)];

fn dfs(
    m: i32,
    n: i32,
    grid: &mut Vec<Vec<i32>>,
    positions: Vec<(i32, i32)>,
    count: &mut i32,
    num: i32,
) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for (i, j) in positions {
        for direct in DIRECTIONS {
            let i = i+direct.0;
            let j = j+direct.1;
            if i < 0 || j < 0 || i == m || j == n {
                continue;
            }
            if grid[i as usize][j as usize] == i32::MAX {
                grid[i as usize][j as usize] = num;
                *count -= 1;
                res.push((i, j));
            }
        }
    }

    res
}

fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
    let mut count = 0;
    let mut positions = Vec::new();
    let (m, n) = (grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == i32::MAX {
                count += 1;
            }
            if grid[i][j] == 0 {
                positions.push((i as i32, j as i32));
            }
        }
    }

    let mut i = 0;
    while count > 0 {
        positions = dfs(m as i32, n as i32, grid, positions, &mut count, i+1);
        i += 1;
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
