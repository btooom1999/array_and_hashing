fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let mut res = 0;
    for i in 0..n {
        res += mat[i][i];
        res += mat[n-i-1][i];
    }

    res - if n*n % 2 == 1 { mat[n/2][n/2] } else { 0 }
}

pub fn main() {
    let mat = [
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ].into_iter().map(Vec::from).collect();
    println!("{}", diagonal_sum(mat));
}
