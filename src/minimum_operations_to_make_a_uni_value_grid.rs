fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut grid = grid.into_iter().flatten().collect::<Vec<_>>();
    grid.sort_unstable();

    let n = grid.len();
    let mut total = 0;
    for i in 0..n {
        if i > 0 && grid[i] % x != grid[i-1] % x {
            return -1;
        }

        total += grid[i];
    }

    let mut res = i32::MAX;
    let mut prefix = 0;
    for i in 0..n {
        let right = (total - prefix - grid[i] - grid[i] * (n-i-1) as i32) / x;
        let left = (grid[i] * i as i32 - prefix) / x;
        res = res.min(left+right);
        prefix += grid[i];
    }

    res
}

pub fn main() {
    let grid = [[2,4],[6,8]].into_iter().map(Vec::from).collect();
    let x = 2;
    println!("{}", min_operations(grid, x));
}
