const DIRECTIONS: [(i32, i32); 8] = [(1,0), (-1,0), (0,1), (0,-1), (-1, -1), (-1, 1), (1,-1), (1,1)];
fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let mut indexes = Vec::new();
    let (m, n) = (board.len(), board[0].len());
    for i in 0..m {
        for j in 0..n {
            let mut live = 0;
            for direct in DIRECTIONS {
                let i = i as i32 + direct.0;
                let j = j as i32 + direct.1;
                if i < 0 || j < 0 || i as usize == m || j as usize == n || board[i as usize][j as usize] == 0 {
                    continue;
                }
                live += 1;
            }
            if board[i][j] == 0 && live == 3 {
                indexes.push((i, j));
            }
            if board[i][j] == 1 && live != 2 && live != 3 {
                indexes.push((i, j));
            }
        }
    }

    while let Some((i, j)) = indexes.pop() {
        board[i][j] = if board[i][j] == 1 { 0 } else { 1 }
    }
}

pub fn main() {
    let mut board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]].into_iter().map(Vec::from).collect();
    game_of_life(&mut board);
    println!("{:?}", board);
}
