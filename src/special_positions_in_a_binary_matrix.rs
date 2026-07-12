fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let (m, n) = (mat.len(), mat[0].len());
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 && !(0..n).any(|k| k != j && mat[i][k] == 1) && !(0..m).any(|k| k != i && mat[k][j] == 1) {
                res += 1;
            }
        }
    }
    res
}

pub fn main() {
    let mat = [[1,0,0],[0,0,1],[1,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", num_special(mat));
}
