fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut res = Vec::new();
    for mut j in 0..n {
        for i in 0..m {
            if (grid[i][j] == 1 && (j+1 == n || grid[i][j+1] == -1)) || (grid[i][j] == -1 && (j == 0 || grid[i][j-1] == 1)) {
                j = usize::MAX;
                break;
            }

            if grid[i][j] == 1 {
                j += 1;
            } else {
                j -= 1;
            }
        }

        res.push(if j == usize::MAX { -1 } else { j as i32 })
    }

    res
}

pub fn main() {
    let grid = [[1,1,1,-1,-1],[1,1,1,-1,-1],[-1,-1,-1,1,1],[1,1,1,1,-1],[-1,-1,-1,-1,-1]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_ball(grid));
}
