fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let (mut first_row, mut last_row) = (0, n-1);
    let (mut first_col, mut last_col) = (0, n-1);
    let (mut i, mut j) = (0, 0);
    let mut direction = 'R';

    let mut res = vec![vec![0; n]; n];
    let mut num = 0;
    while num < n*n {
        num += 1;
        res[i][j] = num as i32;
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
    let n = 3;
    println!("{:?}", generate_matrix(n));
}
