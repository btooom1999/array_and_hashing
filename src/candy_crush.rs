use std::collections::HashSet;

fn candy_crush(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut hashset = HashSet::<(usize, usize)>::new();
    let mut board = board.iter().enumerate().fold(
        vec![vec![0; board.len()]; board[0].len()], 
        |mut vec, (i, b)| {
            for (j, num) in b.iter().enumerate() {
                vec[j][i] = *num;
            }
            vec
        }
    );
    loop {
        let mut count = 0;   

        for (i, col) in board.iter().enumerate() {
            let mut duplicate_counts = 0;
            let mut prev_num = -1;
            for (j, num) in col.iter().enumerate() {
                if *num == 0 {
                    duplicate_counts = 0;
                    prev_num = -1;
                    continue;
                }

                if prev_num == *num {
                    duplicate_counts += 1;
                } else {
                    duplicate_counts = 1;
                    prev_num = *num;
                }

                if duplicate_counts == 3 {
                    count += 1;
                    hashset.insert((i, j-2));
                    hashset.insert((i, j-1));
                    hashset.insert((i, j));
                } else if duplicate_counts > 3 {
                    hashset.insert((i, j));
                }
            }
        }

        for j in 0..board[0].len() {
            let mut duplicate_counts = 0;
            let mut prev_num = -1;
            for i in 0..board.len() {
                let num = board[i][j];
                if num == 0 {
                    duplicate_counts = 0;
                    prev_num = -1;
                    continue;
                }

                if prev_num == num {
                    duplicate_counts += 1;
                } else {
                    duplicate_counts = 1;
                    prev_num = num;
                }

                if duplicate_counts == 3 {
                    count += 1;
                    hashset.insert((i-2, j));
                    hashset.insert((i-1, j));
                    hashset.insert((i, j));
                } else if duplicate_counts > 3 {
                    hashset.insert((i, j));
                }
            }
        }

        if count == 0 {
            break;
        }


        let mut positions = hashset.iter().copied().collect::<Vec<(usize, usize)>>();
        positions.sort_by(|(_, a), (_, b)| a.cmp(b));

        for (i, j) in positions {
            if let Some(vec) = board.get_mut(i) {
                vec.remove(j);
                vec.insert(0, 0);
            }
        }

        hashset.clear();
    }

    board.iter().enumerate().fold(
        vec![vec![0; board.len()]; board[0].len()], 
        |mut vec, (j,b)| {
            for (i, num) in b.iter().enumerate() {
                vec[i][j] = *num;
            }
            vec
        }
    )
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

