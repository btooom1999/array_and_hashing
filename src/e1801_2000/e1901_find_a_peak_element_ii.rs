fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let (m_len, n_len) = (mat.len(), mat[0].len());
    let mut l = 0;
    let mut r = n_len-1;
    while l < r {
        let m = (l+r)/2;
        let mut max_row = 0;
        for i in 1..m_len {
            if mat[i][m] > mat[max_row][m] {
                max_row = i;
            }
        }

        let l_val = if m>0 { mat[max_row][m-1] } else { -1 };
        let r_val = if m+1<n_len { mat[max_row][m+1] } else { -1 };
        if mat[max_row][m] > l_val && mat[max_row][m] > r_val {
            return vec![max_row as i32, m as i32];
        } else if mat[max_row][m] < r_val {
            l = m+1;
        } else {
            r = m-1;
        }
    }

    unreachable!()
}

pub fn main() {
    let mat = [[10,20,15],[21,30,14],[7,16,32]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_peak_grid(mat));
}
