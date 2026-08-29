fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let (m, n) = (mat.len(), mat[0].len());
    let mut prefix = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            prefix[i][j] = mat[i][j];
            if i>0 {
                prefix[i][j] += prefix[i-1][j];
            }
            if j>0 {
                prefix[i][j] += prefix[i][j-1];
            }
            if i>0 && j>0 {
                prefix[i][j] -= prefix[i-1][j-1];
            }
        }
    }

    let is_exist_square = |len: usize| -> bool {
        if len == 0 { return true; };
        for i in len-1..m {
            for j in len-1..n {
                let mut sum = prefix[i][j];
                if i >= len {
                    sum -= prefix[i-len][j];
                }
                if j >= len {
                    sum -= prefix[i][j-len];
                }
                if i >= len && j >= len {
                    sum += prefix[i-len][j-len];
                }
                if sum <= threshold { return true; }
            }
        }

        false
    };

    let mut l = 0;
    let mut r = m.min(n)+1;
    while l < r {
        let m = (l+r)/2;
        if is_exist_square(m) {
            l = m + 1;
        } else {
            r = m;
        }
    }

    (l-1) as i32
}

pub fn main() {
    let mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]].into_iter().map(Vec::from).collect();
    let threshold = 25;
    println!("{}", max_side_length(mat, threshold));
}
