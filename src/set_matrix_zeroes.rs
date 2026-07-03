fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut is_first_col_zero = false;
    let mut is_first_row_zero = false;
    for i in 0..m {
        if matrix[i][0] == 0 {
            is_first_col_zero = true;
        }
    }

    for j in 0..n {
        if matrix[0][j] == 0 {
            is_first_row_zero = true;
        }
    }

    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 0 {
                matrix[0][j] = 0;
                matrix[i][0] = 0;
            }
        }
    }

    for j in 1..n {
        if matrix[0][j] == 0 {
            for i in 0..m {
                matrix[i][j] = 0;
            }
        }
    }

    for i in 1..m {
        if matrix[i][0] == 0 {
            for j in 0..n {
                matrix[i][j] = 0;
            }
        }
    }

    if is_first_col_zero {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }

    if is_first_row_zero {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }
}

pub fn main() {
    let mut matrix = [[1,1,1],[1,0,1],[1,1,1]].into_iter().map(Vec::from).collect();
    set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}
