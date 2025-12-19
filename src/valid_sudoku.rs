use std::{collections::HashSet};

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            let mut hashset: HashSet<char> = HashSet::with_capacity(9);

            // 3x3
            let row1 = board[i*3][j*3..j*3+3].iter().cloned().filter(|v| *v != '.').collect::<Vec<_>>();
            let row2 = board[i*3+1][j*3..j*3+3].iter().cloned().filter(|v| *v != '.').collect::<Vec<_>>();
            let row3 =  board[i*3+2][j*3..j*3+3].iter().cloned().filter(|v| *v != '.').collect::<Vec<_>>();

            for c in row1.into_iter().chain(row2).chain(row3) {
                if hashset.contains(&c) { return false; }
                hashset.insert(c);
            }
        }
    }

    for row in board.iter() {
        let mut hashset: HashSet<char> = HashSet::new();
        for c in row.iter().filter(|v| **v != '.') {
            if hashset.contains(c) { return false; }
            hashset.insert(*c);
        }
    }

    for i in 0..9 {
        let col = board.iter().map(|v| v[i]).filter(|v| *v != '.').collect::<Vec<char>>();
        let mut hashset: HashSet<char> = HashSet::new();
        for c in col.iter() {
            if hashset.contains(c) { return false; }
            hashset.insert(*c);
        }
    }

    true
}

pub fn main () {
    let board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];
    
    // let board = vec![
    //     vec!['.','.','.','.','8','.','.','.','.'],
    //     vec!['.','.','.','.','.','.','5','.','.'],
    //     vec!['.','.','.','.','4','.','.','2','.'],
    //     vec!['.','.','.','3','.','9','.','.','.'],
    //     vec!['.','.','1','8','.','.','9','.','.'],
    //     vec!['.','.','.','.','.','5','1','.','.'],
    //     vec!['.','.','3','.','.','8','.','.','.'],
    //     vec!['.','1','2','.','3','.','.','.','.'],
    //     vec!['.','.','.','.','.','7','.','.','1']
    // ];

    let res = is_valid_sudoku(board);

    println!("sudoku is valid ? - {}", res);
}
