fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let (r, c) = (r as usize, c as usize);
    if r * c != mat.len() * mat[0].len() {
        return mat;
    }
    let mut res = vec![vec![0; c]; r];
    for (order, num) in mat.into_iter().flatten().enumerate() {
        let (i, j) = (order / c, order % c);
        res[i][j] = num;
    }

    res
}

pub fn main() {
    let mat = [[1,2],[3,4]].into_iter().map(Vec::from).collect();
    let r = 1;
    let c = 4;
    println!("{:?}", matrix_reshape(mat, r, c));
}
