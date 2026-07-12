fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let mut res = Vec::new();
    for i in 0..m {
        let j = matrix[i].iter().enumerate().min_by_key(|v| v.1).unwrap().0;
        if (0..m).all(|k| matrix[k][j] <= matrix[i][j]) {
            res.push(matrix[i][j]);
        }
    }

    res
}

pub fn main() {
    let matrix = [[3,7,8],[9,11,13],[15,16,17]].into_iter().map(Vec::from).collect();
    println!("{:?}", lucky_numbers(matrix));
}
