fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (mat.len(), mat[0].len());
    let mut res = Vec::new();
    let (mut i, mut j) = (0, 0);
    let mut reverse = true;
    while i < m && j < n {
        let (mut x, mut y) = (i, j);
        let mut temp = Vec::new();
        while x < m && y < n {
            temp.push(mat[x][y]);
            x += 1;
            y = y.checked_sub(1).unwrap_or(n);
        }
        if j < n-1 {
            j += 1;
        } else {
            i += 1;
        }
        if reverse {
            temp.reverse();
        }
        res.extend(temp);
        reverse = !reverse;
    }

    res
}

pub fn main() {
    let mat = [[1,2,3],[4,5,6],[7,8,9]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_diagonal_order(mat))
}
