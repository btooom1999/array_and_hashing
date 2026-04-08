use std::collections::HashSet;

fn dfs(
    hashset: &mut HashSet<(usize, usize)>,
    m: usize,
    n: usize,
    i: usize,
    j: usize,
    grid: &Vec<Vec<char>>,
) {
    hashset.insert((i, j));

    if i > 0 && grid[i-1][j] == '1' && !hashset.contains(&(i-1, j)) {
        dfs(hashset, m, n, i-1, j, grid);
    }

    if i+1 < m && grid[i+1][j] == '1' && !hashset.contains(&(i+1, j)) {
        dfs(hashset, m, n, i+1, j, grid);
    }

    if j > 0 && grid[i][j-1] == '1' && !hashset.contains(&(i, j-1)) {
        dfs(hashset, m, n, i, j-1, grid);
    }

    if j+1 < n && grid[i][j+1] == '1' && !hashset.contains(&(i, j+1)) {
        dfs(hashset, m, n, i, j+1, grid);
    }
}

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut hashset = HashSet::new();
    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '1' && !hashset.contains(&(i, j)) {
                count += 1;
                dfs(&mut hashset, grid.len(), grid[0].len(), i, j, &grid);
            }
        }
    }

    count
}

pub fn main() {
    let grid = [
        ["1","1","1","1","0"].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        ["1","1","0","1","0"].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        ["1","1","0","0","0"].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        ["0","0","0","0","0"].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
    ];
    println!("{}", num_islands(grid.into()));
}
