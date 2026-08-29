use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 8] = [(1,0), (-1,0), (0,1), (0,-1), (-1,-1), (-1,1), (1,-1), (1,1)];

fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
    if board[click[0] as usize][click[1] as usize] == 'M' {
        board[click[0] as usize][click[1] as usize] = 'X';
        return board;
    }

    let (m, n) = (board.len() as i32, board[0].len() as i32);
    let mut queue = VecDeque::from([(click[0], click[1])]);
    while let Some((i, j)) = queue.pop_front() {
        let mut bomb = 0;
        let mut indexes = Vec::new();
        for direct in DIRECTIONS {
            let i = i + direct.0;
            let j = j + direct.1;
            if i < 0 || j < 0 || i == m || j == n {
                continue;
            }
            let i = i as usize;
            let j = j as usize;
            if board[i][j] == 'M' {
                bomb += 1;
            } else if board[i][j] == 'E' {
                indexes.push((i, j));
            }
        }
        if bomb == 0 {
            board[i as usize][j as usize] = 'B';
            while let Some((i, j)) = indexes.pop() {
                board[i][j] = 'B';
                queue.push_back((i as i32, j as i32));
            }
        } else {
            board[i as usize][j as usize] = (bomb + b'0') as char;
        }
    }

    board
}

pub fn main() {
    let board = [["E","E","E","E","E"],["E","E","M","E","E"],["E","E","E","E","E"],["E","E","E","E","E"]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    let click = vec![3, 0];
    println!("{:?}", update_board(board, click));
}
