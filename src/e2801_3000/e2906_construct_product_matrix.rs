const MOD: i32 = 12345;
fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut prefix = vec![vec![1; n]; m];

    for num in 1..m*n {
        let i = num / n;
        let j = num % n;
        if j == 0 {
            prefix[i][j] = prefix[i-1][n-1] * (grid[i-1][n-1] % MOD) % MOD;
        } else {
            prefix[i][j] = prefix[i][j-1] * (grid[i][j-1] % MOD) % MOD;
        }
    }


    let mut product = 1;
    for num in (0..m*n).rev() {
        let i = num / n;
        let j = num % n;
        prefix[i][j] = prefix[i][j] * product % MOD;
        product = product * (grid[i][j] % MOD) % MOD;
    }

    prefix
}

pub fn main() {
    let grid = [[100_000, 100_000],[100_000, 100_000], [100_000, 100_000]].into_iter().map(Vec::from).collect();
    println!("{:?}", construct_product_matrix(grid));
}
