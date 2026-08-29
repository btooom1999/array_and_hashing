fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut cols = vec![0; n];
    for i in 0..m {
        let mut flip = false;
        for j in 0..n {
            if !flip && grid[i][0] == 0 {
                flip = true;
            }
            if flip {
                grid[i][j] = (grid[i][j] == 0) as i32;
            }

            cols[j] += (grid[i][j] == 0) as i32;
        }
    }

    for j in 0..n {
        for i in 0..m {
            if cols[j] > (m/2) as i32 {
                grid[i][j] = (grid[i][j] == 0) as i32;
            }
        }
    }

    let mut res = 0;
    for i in 0..m {
        let mut num = 0;
        for (pos, j) in (0..n).rev().enumerate() {
            num |= grid[i][j] << pos as i32;
        }

        res += num;
    }

    res
}

pub fn main() {
    let grid = [[0,0,1,1],[1,0,1,0],[1,1,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", matrix_score(grid));
}
