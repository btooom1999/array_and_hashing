fn multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = b[0].len();
    let mut res = vec![vec![0; m]; n];
    for (i, vec) in a.iter().enumerate() {
        for (j, num) in vec.iter().enumerate() {
            for x in 0..m {
                res[i][x] += num * b[j][x];
            }
        }
    }

    res
}

pub fn main() {
    let a = vec![vec![1], vec![2]];
    let b = vec![vec![3, 4, 5]];

    println!("{:?}", multiply(a, b));
}
