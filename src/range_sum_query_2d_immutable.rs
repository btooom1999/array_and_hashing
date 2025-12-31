#[derive(Debug)]
struct NumMatrix(Vec<Vec<i32>>);

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut res = vec![vec![0; n]; m];
        for i in 0..m {
            let mut sum = matrix[i].iter().sum::<i32>();
            for j in (0..n).rev() {
                res[i][j] = sum;
                sum -= matrix[i][j];
            }
        }

        for j in 0..n {
            let mut sum = 0;
            for i in 0..m {
                sum += res[i][j];
                res[i][j] = sum;
            }
        }

        Self(res)
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        let sum1 = if row1 == 0 {
            self.0[row2][col2]
        } else {
            self.0[row2][col2] - self.0[row1 - 1][col2]
        };

        let sum2 = if col1 == 0 {
            0
        } else if row1 == 0 {
            self.0[row2][col1 - 1]
        } else {
            self.0[row2][col1 - 1] - self.0[row1 - 1][col1 - 1]
        };

        sum1 - sum2
    }
}

pub fn main() {
    let num_matrix = NumMatrix::new(vec![vec![1, 2], vec![3, 4]]);

    println!("{}", num_matrix.sum_region(0, 1, 1, 1));
}
