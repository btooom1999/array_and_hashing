fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    let n = matrix.len();
    for i in 0..n {
        let mut temp = vec![(false, false); n+1];
        temp[0] = (true, true);
        for j in 0..n {
            temp[matrix[i][j] as usize].0 = true;
            temp[matrix[j][i] as usize].1 = true;
        }

        if temp.iter().any(|&v| !v.0 || !v.1) {
            return false;
        }
    }

    true
}

pub fn main() {
    let matrix = [[1,2,3],[3,1,2],[2,3,1]].into_iter().map(Vec::from).collect();
    println!("{}", check_valid(matrix));
}
