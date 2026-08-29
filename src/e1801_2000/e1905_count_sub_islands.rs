const DIRECTIONS: [(i32, i32); 4] = [(0,1), (0,-1), (1,0), (-1,0)];

fn dfs(
    i: i32,
    j: i32,
    m: i32,
    n: i32,
    valid: &mut bool,
    grid1: &Vec<Vec<i32>>,
    grid2: &mut Vec<Vec<i32>>,
) {
    grid2[i as usize][j as usize] = 0;

    for direct in DIRECTIONS {
        let i = direct.0 + i;
        let j = direct.1 + j;

        if i < 0 || i == m || j < 0 || j == n || grid2[i as usize][j as usize] == 0 {
            continue;
        }
        if grid2[i as usize][j as usize] == 1 && grid1[i as usize][j as usize] == 0 {
            *valid = false;
        }

        dfs(i, j, m, n, valid, grid1, grid2);
    }
}

fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let m = grid2.len();
    let n = grid2[0].len();
    for i in 0..m {
        for j in 0..n {
            if grid2[i][j] == 1 && grid1[i][j] == 1 {
                let mut valid = true;
                dfs(i as i32, j as i32, m as i32, n as i32, &mut valid, &grid1, &mut grid2);
                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn main() {
    let grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]].into_iter().map(Vec::from).collect();
    let grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]].into_iter().map(Vec::from).collect();
    println!("{}", count_sub_islands(grid1, grid2));
}
