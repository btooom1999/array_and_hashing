fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut grid = grid.into_iter().flatten().collect::<Vec<_>>();
    grid.sort_unstable();

    let remaining = grid[0] % x;
    if grid.iter().any(|&num| num % x != remaining) {
        return -1;
    }

    let n = grid.len();
    grid.iter().fold(0, |acc, &num| acc + (num - grid[n/2]).abs() / x)
}

pub fn main() {
    let grid = [[2,4],[6,8]].into_iter().map(Vec::from).collect();
    let x = 2;
    println!("{}", min_operations(grid, x));
}
