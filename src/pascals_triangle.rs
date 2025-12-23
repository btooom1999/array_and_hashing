fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        return vec![vec![1]];
    }
    if num_rows == 2 {
        return vec![vec![1], vec![1, 1]];
    }

    let mut res = vec![vec![1], vec![1, 1]];
    for i in 2..num_rows as usize {
        let mut row = vec![1; i + 1];
        for j in 1..i {
            row[j] = res[i - 1][j] + res[i - 1][j - 1];
        }

        res.push(row);
    }

    res
}

pub fn main() {
    let num_rows = 5;
    println!("{:?}", generate(num_rows));
}
