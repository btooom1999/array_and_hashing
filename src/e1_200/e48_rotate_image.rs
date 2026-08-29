fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..n {
        for j in i+1..n {
            (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
        }
    }

    for i in 0..n {
        matrix[i].reverse();
    }
}

pub fn main() {
    // let mut matrix = [[1,-2,3],[-4,5,-6],[-7,-8,-9]].into_iter().map(Vec::from).collect();
    let mut matrix = [[1000,-1000,-1000],[-1000,-1000,1000],[1000,-1000,1000]].into_iter().map(Vec::from).collect();
    rotate(&mut matrix);
    println!("{:?}", matrix);
}
