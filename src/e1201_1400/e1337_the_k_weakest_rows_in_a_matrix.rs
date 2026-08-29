fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let (m, n, k) = (mat.len(), mat[0].len(), k as usize);
    let mut res = Vec::with_capacity(m);
    for i in 0..m {
        let mut count = 0;
        for j in 0..n {
            if mat[i][j] == 0 { break; }
            count += 1;
        }
        res.push((i, count));
    }

    res.sort_by_key(|x| x.1);
    res[..k].iter().map(|v| v.0 as i32).collect()
}

pub fn main() {
    let mat = [
        [1,1,0,0,0],
        [1,1,1,1,0],
        [1,0,0,0,0],
        [1,1,0,0,0],
        [1,1,1,1,1]
    ].into_iter().map(Vec::from).collect();
    let k = 3;
    println!("{:?}", k_weakest_rows(mat, k));
}
