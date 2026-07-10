fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut res = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            res[i][j] = matrix[j][i];
        }
    }

    res
}

pub fn main() {
    let matrix = [[1,2,3],[4,5,6],[7,8,9]].into_iter().map(Vec::from).collect();
    println!("{:?}", transpose(matrix));
}
