const DIRECTION: [(i32, i32); 4] = [(0,-1), (0,1), (-1,0), (1,0)];

fn dfs(
    m: i32,
    n: i32,
    i: i32,
    j: i32,
    changable_cells: &mut Vec<Vec<bool>>,
    board: &mut Vec<Vec<char>>,
) {
    changable_cells[i as usize][j as usize] = false;
    for direct in DIRECTION {
        let i = direct.0 + i;
        let j = direct.1 + j;
        if i < 0 || i == m || j < 0 || j == n || !changable_cells[i as usize][j as usize] || board[i as usize][j as usize] == 'X' {
            continue;
        }

        dfs(m, n, i, j, changable_cells, board);
    }
}

fn solve(board: &mut Vec<Vec<char>>) {
    let (m, n) = (board.len(), board[0].len());
    let mut changable_cells = vec![vec![true; n]; m];

    for i in 0..m {
        for j in 0..n {
            if (i == 0 || j == 0 || i+1 == m || j+1 == n) && changable_cells[i][j] && board[i][j] == 'O' {
                dfs(m as i32, n as i32, i as i32, j as i32, &mut changable_cells, board);
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            if changable_cells[i][j] && board[i][j] == 'O' {
                board[i][j] = 'X';
            }
        }
    }
}

pub fn main() {
    let mut board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
        .into_iter()
        .map(|row| row.into_iter().map(|c| c.chars().next().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    solve(&mut board);
    println!("{:?}", board)
}
