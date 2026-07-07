fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let n = matrix[0].len();
    let mut heights = vec![0; n];
    for row in matrix {
        for i in 0..n {
            if row[i] == 1 {
                heights[i] += 1;
            } else {
                heights[i] = 0;
            }
        }

        let mut temp = heights.clone();
        temp.sort_unstable();

        for i in 0..n {
            res = res.max(temp[i] * (n-i) as i32);
        }
    }

    res
}

pub fn main() {
    let matrix = [[0,0,1],[1,1,1],[1,0,1]].into_iter().map(Vec::from).collect();
    println!("{}", largest_submatrix(matrix));
}
