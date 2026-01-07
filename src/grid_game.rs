fn grid_game(mut grid: Vec<Vec<i32>>) -> i64 {
    let mut sum1 = grid[0].iter().map(|v| *v as i64).sum::<i64>() - *grid.first().unwrap().first().unwrap() as i64;
    let mut sum2 = 0;
    let mut res = sum1;

    let n = grid[0].len();
    let mut i = 1;
    while i < n {
        sum1 -= grid[0][i] as i64;
        sum2 += grid[1][i-1] as i64;

        res = std::cmp::min(res, std::cmp::max(sum1, sum2));

        i += 1;
    }

    res
}

pub fn main() {
    let grid = vec![vec![20,3,20,17,2,12,15,17,4,15],vec![20,10,13,14,15,5,2,3,14,3]];
    println!("{}", grid_game(grid));
}
