fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let (m, n, k) = (matrix.len(), matrix[0].len(), k as usize);
    let mut prefix = vec![vec![0; n]; m];
    let mut res = Vec::new();

    for i in 0..m {
        for j in 0..n {
            prefix[i][j] = matrix[i][j];
            if i > 0 && j > 0 {
                prefix[i][j] ^= prefix[i-1][j-1];
            }
            if i > 0 {
                prefix[i][j] ^= prefix[i-1][j];
            }
            if j > 0 {
                prefix[i][j] ^= prefix[i][j-1];
            }

            res.push(prefix[i][j]);
        }
    }

    res.sort();
    res[res.len() - k]
}

pub fn main() {
    let matrix = [[5,2,1],[1,6,1]].into_iter().map(Vec::from).collect();
    let k = 1;
    println!("{}", kth_largest_value(matrix, k));
}
