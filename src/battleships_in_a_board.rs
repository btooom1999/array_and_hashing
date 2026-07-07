const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn dfs(
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    m: usize,
    n: usize,
) {
    visited[i][j] = true;
    for direct in DIRECTIONS {
        let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.checked_sub(1).unwrap_or(m) };
        let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.checked_sub(1).unwrap_or(n) };
        if ni < m && nj < n && board[ni][nj] == 'X' && !visited[ni][nj] {
            dfs(board, visited, ni, nj, m, n);
        }
    }
}

fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let (m, n) = (board.len(), board[0].len());
    let mut visited = vec![vec![false; n]; m];
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 'X' && !visited[i][j] {
                count += 1;
                dfs(&board, &mut visited, i, j, m, n);
            }
        }
    }

    count
}

pub fn main() {
    let board = [["X",".",".","X"],[".",".",".","X"],[".",".",".","X"]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    println!("{}", count_battleships(board));
}
