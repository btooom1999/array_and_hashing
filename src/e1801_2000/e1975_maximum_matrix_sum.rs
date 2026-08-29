fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut min = i64::MAX;
    let mut negative_counts = 0;

    let mut res = 0;
    let n = matrix.len();
    for i in 0..n {
        for j in 0..n {
            let val = matrix[i][j] as i64;
            res += val.abs();
            min = min.min(val.abs());
            if val < 0 {
                negative_counts += 1;
            }
        }
    }

    if negative_counts % 2 == 1 {
        res = res - min - min;
    }

    res
}

pub fn main() {
    let matrix = [[1,-1],[-1,1]].into_iter().map(Vec::from).collect();
    println!("{}", max_matrix_sum(matrix));
}
