fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    // (0, 0, 0, 0) = (DOWN, RIGHT, DIAGONAL_LEFT, DIAGONAL_RIGHT)
    let mut prefix = vec![vec![(0,0,0,0); n+2]; m+2];
    for i in 0..m {
        for j in 0..n {
            let val = grid[i][j];
            prefix[i+1][j+1] = (val, val, val, val);
            prefix[i+1][j+1].0 += prefix[i][j+1].0;
            prefix[i+1][j+1].1 += prefix[i+1][j].1;
            prefix[i+1][j+1].2 += prefix[i][j].2;
            prefix[i+1][j+1].3 += prefix[i][j+2].3;
        }
    }

    let mut max = 0;
    for i in 1..=m {
        for j in 1..=n {
            for k in max+1..=m.min(n) {
                if i+k <= m && j+k <= n {
                    let target = prefix[i+k][j].0 - prefix[i-1][j].0;
                    if (j..=j+k).any(|j| prefix[i+k][j].0 - prefix[i-1][j].0 != target) {
                        continue;
                    }
                    if (i..=i+k).any(|i| prefix[i][j+k].1 - prefix[i][j-1].1 != target) {
                        continue;
                    }
                    if prefix[i+k][j+k].2 - prefix[i-1][j-1].2 != target {
                        continue;
                    }
                    if prefix[i+k][j].3 - prefix[i-1][j+k+1].3 != target {
                        continue;
                    }
                    max = k;
                }
            }
        }
    }

    (max+1) as i32
}

pub fn main() {
    let grid = [[7,1,4,5,6],[2,5,1,6,4],[1,5,4,3,2],[1,2,7,3,4]].into_iter().map(Vec::from).collect();
    println!("{}", largest_magic_square(grid));
}
