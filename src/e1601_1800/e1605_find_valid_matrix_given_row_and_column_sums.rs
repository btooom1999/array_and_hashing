fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let (m, n) = (row_sum.len(), col_sum.len());
    let mut res = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let min = row_sum[i].min(col_sum[j]);
            res[i][j] = min;
            row_sum[i] -= min;
            col_sum[j] -= min;
        }
    }

    res
}

pub fn main() {
    let row_sum = [43,6,0].to_vec();
    let col_sum = [14,2,14,4,15].to_vec();
    println!("{:?}", restore_matrix(row_sum, col_sum));
}
