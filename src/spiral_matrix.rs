fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut first_row, mut last_row) = (0, m-1);
    let (mut first_col, mut last_col) = (0, n-1);
    let (mut i, mut j) = (0, 0);
    let mut direction = 'R';

    let mut res = Vec::new();
    while res.len() != m*n {
        res.push(matrix[i][j]);
        match direction {
            'R' => {
                if j == last_col {
                    first_row += 1;
                    direction = 'D';
                    i += 1;
                } else {
                    j += 1;
                }
            }
            'D' => {
                if i == last_row {
                    last_col -= 1;
                    direction = 'L';
                    j -= 1;
                } else {
                    i += 1;
                }
            }
            'L' => {
                if j == first_col {
                    last_row -= 1;
                    direction = 'U';
                    i -= 1;
                } else {
                    j -= 1;
                }
            }
            'U' => {
                if i == first_row {
                    first_col += 1;
                    direction = 'R';
                    j += 1;
                } else {
                    i -= 1;
                }
            }
            _ => unreachable!(),
        }
    }

    res
}

pub fn main() {
    let matrix = [[1,2,3],[4,5,6],[7,8,9]].into_iter().map(Vec::from).collect();
    println!("{:?}", spiral_order(matrix));
}
