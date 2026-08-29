fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut prefix = vec![vec![(0,0); n+1]; m+1];

    for i in 0..m {
        for j in 0..n {
            prefix[i+1][j+1] = prefix[i+1][j];
            if grid[i][j] == 'X' {
                prefix[i+1][j+1].0 += 1;
            } else if grid[i][j] == 'Y' {
                prefix[i+1][j+1].1 += 1;
            }
        }
    }

    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            prefix[i+1][j+1].0 += prefix[i][j+1].0;
            prefix[i+1][j+1].1 += prefix[i][j+1].1;
            if prefix[i+1][j+1].0 > 0 && prefix[i+1][j+1].0 == prefix[i+1][j+1].1 {
                res += 1;
            }
        }
    }

    res
}

pub fn main() {
    let grid = [
        [".",".","."],
        [".","X","X"],
        ["Y",".","."],
        ["X",".","."]
    ].map(|v| v.map(|v| v.chars().next().unwrap()).to_vec()).to_vec();
    println!("{}", number_of_submatrices(grid));
}
