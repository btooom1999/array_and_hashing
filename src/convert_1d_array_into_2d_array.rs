fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    let (m, n) = (m as usize, n as usize);
    if original.len() != m * n { return Vec::new() };
    let mut res = vec![vec![0; n]; m];
    for (order, num) in original.into_iter().enumerate() {
        let (i, j) = (order/n, order%n);
        res[i][j] = num;
    }
    res
}

pub fn main() {
    let original = [1,2,3,4].to_vec();
    let m = 2;
    let n = 2;
    println!("{:?}", construct2_d_array(original, m, n));
}
