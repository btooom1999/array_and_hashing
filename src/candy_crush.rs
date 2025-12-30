use std::{collections::HashSet, usize};

fn candy_crush(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = board.len();
    let n = board[0].len();

    loop {
        let mut can_crush = false;

        // check horizontally
        for i in 0..m {
            for j in 0..n-2 {
                let v = board[i][j].abs();
                if v != 0 && v == board[i][j+1].abs() && v == board[i][j+2].abs() {
                    can_crush = true;
                    board[i][j] = -v;
                    board[i][j+1] = -v;
                    board[i][j+2] = -v;
                }
            }
        }

        // check vertically
        for j in 0..n {
            for i in 0..m-2 {
                let v = board[i][j].abs();
                if v != 0 && v == board[i+1][j].abs() && v == board[i+2][j].abs() {
                    can_crush = true;
                    board[i][j] = -v;
                    board[i+1][j] = -v;
                    board[i+2][j] = -v;
                }
            }
        }

        if !can_crush {
            break;
        }

        // crush 
        for j in 0..n {
            let mut w = m as i32 - 1;
            for i in (0..m).rev() {
                if board[i][j] > 0 {
                    board[w as usize][j] = board[i][j];
                    w -= 1;
                }
            }

            while w >= 0 {
                board[w as usize][j] = 0;
                w -= 1;
            }
        }
    }

    board
}

pub fn main() {
    let board = vec![
        vec![110,  5, 112, 113, 114],
        vec![210,211,   5, 213, 214],
        vec![310,311,   3, 313, 314],
        vec![410,411, 412,   5, 414],
        vec![  5,  1, 512,   3,   3],
        vec![610,  4,   1, 613, 614],
        vec![710,  1,   2, 713, 714],
        vec![810,  1,   2,   1,   1],
        vec![  1,  1,   2,   2,   2],
        vec![  4,  1,   4,   4,1014],
    ];

    println!("{:?}", candy_crush(board));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let board = vec![ 
            vec![110, 5, 112, 113, 114], 
            vec![210, 211, 5, 213, 214], 
            vec![310, 311, 3, 313, 314], 
            vec![410, 411, 412, 5, 414], 
            vec![5, 1, 512, 3, 3], 
            vec![610, 4, 1, 613, 614], 
            vec![710, 1, 2, 713, 714], 
            vec![810, 1, 2, 1, 1], 
            vec![1, 1, 2, 2, 2], 
            vec![4, 1, 4, 4, 1014], 
        ]; 
        let expected = vec![ 
            vec![0, 0, 0, 0, 0], 
            vec![0, 0, 0, 0, 0], 
            vec![0, 0, 0, 0, 0], 
            vec![110, 0, 0, 0, 114], 
            vec![210, 0, 0, 0, 214], 
            vec![310, 0, 0, 113, 314], 
            vec![410, 0, 0, 213, 414], 
            vec![610, 211, 112, 313, 614], 
            vec![710, 311, 412, 613, 714], 
            vec![810, 411, 512, 713,1014], 
        ]; 
        assert_eq!(candy_crush(board), expected);   
    }

    #[test]
    fn test_case2() {
        let board = vec![
            vec![1, 1, 1],
            vec![2, 3, 4],
            vec![5, 6, 7],
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![2, 3, 4],
            vec![5, 6, 7],
        ];
        assert_eq!(candy_crush(board), expected);
    }

    #[test]
    fn test_case3() {
        let board = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(candy_crush(board.clone()), board.clone());
    }

    #[test]
    fn test_case4() {
        let board = vec![
            vec![5, 5, 5],
            vec![5, 0, 0],
            vec![5, 0, 0],
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(candy_crush(board), expected);
    }

    #[test]
    fn test_case5() {
        let board = vec![
            vec![7, 7, 7],
            vec![7, 7, 7],
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(candy_crush(board), expected);
    }

    #[test]
    fn test_case6() {
        let board = vec![
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(candy_crush(board), expected);
    }

    #[test]
    fn test_case7() {
        let board = vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![3, 3, 3, 3],
        ];
        let expected = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(candy_crush(board), expected);
    }

    #[test]
    fn test_case8() {
        let board = vec![
            vec![1, 2, 2],
            vec![1, 2, 2],
            vec![1, 2, 2],
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(candy_crush(board), expected);
    }

    #[test]
    fn test_case9() {
        let board = vec![
            vec![3, 3, 3],
            vec![3, 4, 5],
            vec![3, 6, 7],
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![0, 4, 5],
            vec![0, 6, 7],
        ];
        assert_eq!(candy_crush(board), expected);
    }

    #[test]
    fn test_case10() {
        let board = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1],
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(candy_crush(board), expected);
    }
}

