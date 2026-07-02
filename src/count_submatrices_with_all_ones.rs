fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (mat.len(), mat[0].len());
    let mut prefix = vec![vec![0; n+1]; m+1];

    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 {
                prefix[i+1][j+1] = prefix[i+1][j] + 1;
            }
        }
    }

    let mut res = 0;
    for i in 1..=m {
        for j in 1..=n {
            if mat[i-1][j-1] == 0 {
                continue;
            }

            let mut width = i32::MAX;
            for k in (1..=i).rev() {
                if prefix[k][j] == 0 {
                    break;
                }

                width = width.min(prefix[k][j]);
                res += width;
            }
        }
    }

    res
}

pub fn main() {
    let mat = [[1,0,1],[1,1,0],[1,1,0]].into_iter().map(Vec::from).collect();
    println!("{}", num_submat(mat));
}
