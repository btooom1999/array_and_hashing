fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let sum = colsum.iter().sum();
    if upper + lower != sum {
        return vec![];
    }

    let n = colsum.len();
    let mut res = vec![vec![0; n]; 2];
    let mut total = 0;
    for i in 0..n {
        if colsum[i] == 2 && upper > 0 && lower > 0 {
            res[0][i] = 1;
            res[1][i] = 1;
            upper -= 1;
            lower -= 1;
            total += 2;
        }
    }

    for i in 0..n {
        if upper > 0 && res[0][i]+res[1][i] < colsum[i] {
            res[0][i] = 1;
            upper -= 1;
            total += 1;
        }

        if lower > 0 && res[1][i]+res[0][i] < colsum[i] {
            res[1][i] = 1;
            lower -= 1;
            total += 1;
        }
    }

    if sum != total { return vec![] }

    res
}

pub fn main() {
    let upper = 2;
    let lower = 1;
    let colsum = [1,1,1].to_vec();
    println!("{:?}", reconstruct_matrix(upper, lower, colsum));
}
