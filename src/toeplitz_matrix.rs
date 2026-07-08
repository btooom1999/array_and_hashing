fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let (m, n) = (matrix.len(), matrix[0].len());
    for i in 0..m {
        for j in 0..n {
            if i+1 < m && j+1 < n && matrix[i][j] != matrix[i+1][j+1] {
                return false;
            }
        }
    }

    true
}

pub fn main() {
    let matrix = [[1,2,3,4],[5,1,2,3],[9,5,1,2]].into_iter().map(Vec::from).collect();
    println!("{}", is_toeplitz_matrix(matrix));
}
