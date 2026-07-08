const DIRECTIONS: [[(i32, i32,); 2]; 8] = [
    [(-1, 0), (-2, 0)],
    [(1, 0), (2, 0)],
    [(0, -1), (0, -2)],
    [(0, 1), (0, 2)],
    [(-1, -1), (-2, -2)],
    [(-1, 1), (-2, 2)],
    [(1, 1), (2, 2)],
    [(1, -1), (2, -2)],
];

fn valid_tic_tac_toe(board: Vec<String>) -> bool {
    let board = board.into_iter().map(|v| v.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let (m, n) = (board.len(), board[0].len());
    let (mut x, mut o) = (0i32, 0i32);
    let (mut x_win, mut o_win) = (false, false);
    for i in 0..m {
        for j in 0..n {
            if board[i][j] != ' ' {
                if board[i][j] == 'X' { x += 1 }
                else if board[i][j] == 'O' { o += 1 }
                for directs in DIRECTIONS {
                    if directs.iter().all(|&(x, y)| {
                        let ni = i as i32 + x;
                        let nj = j as i32 + y;
                        !(ni < 0 || nj < 0 || ni >= m as i32 || nj >= n as i32|| board[ni as usize][nj as usize] != board[i][j])
                    }) {
                        if board[i][j] == 'X' { x_win = true; }
                        else { o_win = true; }
                    }
                }
            }
        }
    }

    if (x == 0 && o > 0) || (o_win && x_win) { return false; }

    if o_win {
        if o==x { return true };
        return false;
    }

    if x_win {
        if x-o == 1 { return true };
        return false;
    }

    x-o >= 0 && x-o <= 1
}

pub fn main() {
    let board = ["XOX","O O","XOX"].into_iter().map(String::from).collect();
    println!("{}", valid_tic_tac_toe(board));
}
