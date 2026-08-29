fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            for k in 1..=i {
                if i >= k && j >= k && mat[i-k][j-k] > mat[i-k+1][j-k+1] {
                    (mat[i-k][j-k], mat[i-k+1][j-k+1]) = (mat[i-k+1][j-k+1], mat[i-k][j-k]);
                } else {
                    break;
                }
            }
        }
    }

    mat
}

pub fn main() {
    let mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", diagonal_sort(mat));
}
