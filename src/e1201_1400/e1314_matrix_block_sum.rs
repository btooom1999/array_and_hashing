fn matrix_block_sum(mut mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let mut prefix_mat = vec![vec![0; n + 1]; m + 1];

    for i in 0..m {
        for j in 0..n {
            prefix_mat[i + 1][j + 1] =
                mat[i][j] + prefix_mat[i][j + 1] + prefix_mat[i + 1][j] - prefix_mat[i][j];
        }
    }

    let sum_block = |r1: usize, c1: usize, r2: usize, c2: usize| -> i32 {
        prefix_mat[r2][c2] - prefix_mat[r1 - 1][c2] - prefix_mat[r2][c1 - 1]
            + prefix_mat[r1 - 1][c1 - 1]
    };

    for i in 0..m {
        for j in 0..n {
            let r1 = std::cmp::max(i as i32 + 1 - k, 1) as usize;
            let c1 = std::cmp::max(j as i32 + 1 - k, 1) as usize;
            let r2 = std::cmp::min(i + 1 + k as usize, m);
            let c2 = std::cmp::min(j + 1 + k as usize, n);
            mat[i][j] = sum_block(r1, c1, r2, c2);
        }
    }

    mat
}

pub fn main() {
    let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let k = 2;
    for m in matrix_block_sum(mat, k) {
        println!("{:?}", m);
    }
}
