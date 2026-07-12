fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
    let total = grid.iter().map(|v| v.iter().sum::<i32>()).sum::<i32>();
    let mut sum = 0;
    let n = grid.len();
    for i in 0..n {
        if grid[i][i] == 0 || grid[n-i-1][i] == 0 { return false; }
        sum += grid[i][i];
        sum += if n-i-1 != i { grid[n-i-1][i] } else { 0 };
    }

    total == sum
}

pub fn main() {
    let grid = [[2,0,0,1],[0,3,1,0],[0,5,2,0],[4,0,0,2]].into_iter().map(Vec::from).collect();
    println!("{}", check_x_matrix(grid));
}
