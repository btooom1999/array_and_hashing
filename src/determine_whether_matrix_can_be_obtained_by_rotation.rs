fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    let n = mat.len();

    for _ in 0..3 {
        if mat == target { return true; }
        let mut temp = mat.clone();
        for i in 0..n {
            for j in 0..n {
                temp[j][i] = mat[n-i-1][j];
            }
        }

        mat = temp;
    }

    mat == target
}

pub fn main() {
    let mat = [[0,0,0],[0,1,0],[1,1,1]].into_iter().map(Vec::from).collect();
    let target = [[1,1,1],[0,1,0],[0,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", find_rotation(mat, target));
}
