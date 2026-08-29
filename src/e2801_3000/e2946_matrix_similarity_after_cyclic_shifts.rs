fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    let (m, n) = (mat.len(), mat[0].len());
    let k = k as usize % n;
    for i in 0..m {
        let mut temp = mat[i].clone();
        if i % 2 == 0 {
            temp.rotate_left(k);
        } else {
            temp.rotate_right(k);
        }

        if temp != mat[i] {
            return false;
        }
    }

    true
}

pub fn main() {
    let mat = [[1,2,3],[4,5,6],[7,8,9]].into_iter().map(Vec::from).collect();
    let k = 4;
    println!("{}", are_similar(mat, k));
}
