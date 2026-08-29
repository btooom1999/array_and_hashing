const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn dfs(
    grid: &Vec<Vec<char>>,
    len: &mut Vec<Vec<i32>>,
    i: usize,
    j: usize,
    m: usize,
    n: usize,
) -> bool {
    for direct in DIRECTIONS {
        let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
        let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
        if ni < m && nj < n && grid[i][j] == grid[ni][nj] {
            if len[ni][nj] != -1 && len[i][j]-len[ni][nj]+1 >= 4 {
                return true;
            }

            if len[ni][nj] == -1 {
                len[ni][nj] = len[i][j].max(0) + 1;
                if dfs(grid, len, ni, nj, m, n) {
                    return true;
                }
            }
        }
    }

    false
}

fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![-1; n]; m];
    for i in 0..m {
        for j in 0..n {
            if visited[i][j] == -1 && dfs(&grid, &mut visited, i, j, m, n) {
                return true;
            }
        }
    }

    false
}

pub fn main() {
    let grid = [["a","a","a","a"],["a","b","b","a"],["a","b","b","a"],["a","a","a","a"]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    println!("{}", contains_cycle(grid));
}
