fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let (m, n) = (grid.len(), grid[0].len());
    let mut row = m-1;
    let mut col = 0;
    while row < m && col < n {
        if grid[row][col] >= 0 {
            count += row + 1;
            col += 1;
        } else {
            row = row.wrapping_sub(1);
        }
    }

    (m * n - count) as i32
}

pub fn main() {
    let grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]].into_iter().map(Vec::from).collect();
    println!("{}", count_negatives(grid));
}
