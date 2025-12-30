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
            self.0[row2][col1-1]
        } else {
            self.0[row2][col1-1] - self.0[row1-1][col1-1]
        };

        sum1 - sum2
    }
}

pub fn main() {
    // let num_matrix = NumMatrix::new(vec![
    //     vec![3, 0, 1, 4, 2], 
    //     vec![5, 6, 3, 2, 1], 
    //     vec![1, 2, 0, 1, 5],
    //     vec![4, 1, 0, 1, 7], 
    //     vec![1, 0, 3, 0, 5]
    // ]);

            let num_matrix = NumMatrix::new(vec![
            vec![1, 2],
            vec![3, 4],
        ]);


    println!("{}", num_matrix.sum_region(0, 1, 1, 1));
    // println!("{}", num_matrix.sum_region(2, 1, 4, 3));
    // println!("{}", num_matrix.sum_region(1, 1, 2, 2));
    // println!("{}", num_matrix.sum_region(1, 2, 2, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let matrix = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(2, 1, 4, 3), 8);
        assert_eq!(nm.sum_region(1, 1, 2, 2), 11);
        assert_eq!(nm.sum_region(1, 2, 2, 4), 12);
    }

    #[test]
    fn test_case2() {
        let matrix = vec![vec![1]];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 0, 0), 1);
    }

    #[test]
    fn test_case3() {
        let matrix = vec![vec![1, 2, 3]];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 0, 2), 6);
    }

    #[test]
    fn test_case4() {
        let matrix = vec![vec![1], vec![2], vec![3]];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 2, 0), 6);
    }

    #[test]
    fn test_case5() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 1, 1), 10);
        assert_eq!(nm.sum_region(0, 1, 1, 1), 6);
    }

    #[test]
    fn test_case6() {
        let matrix = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 2, 2), 0);
    }

    #[test]
    fn test_case7() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 2, 2), 45);
        assert_eq!(nm.sum_region(1, 1, 2, 2), 28);
    }

    #[test]
    fn test_case8() {
        let matrix = vec![
            vec![5, -4, 3],
            vec![-1, 2, -2],
            vec![4, 0, -3],
        ];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 2, 2), 4);
        assert_eq!(nm.sum_region(1, 1, 2, 2), -3);
    }

    #[test]
    fn test_case9() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 2, 3), 78);
        assert_eq!(nm.sum_region(1, 1, 2, 2), 34);
    }

    #[test]
    fn test_case10() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        let nm = NumMatrix::new(matrix);
        assert_eq!(nm.sum_region(0, 0, 2, 1), 21);
        assert_eq!(nm.sum_region(1, 0, 2, 1), 18);
    }
}

