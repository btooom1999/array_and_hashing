fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let mut queue = std::collections::VecDeque::new();
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == 'R' {
                queue.push_back((i as i32, j as i32, -1, 0));
                queue.push_back((i as i32, j as i32, 1, 0));
                queue.push_back((i as i32, j as i32, 0, -1));
                queue.push_back((i as i32, j as i32, 0, 1));
                break;
            }
        }
    }

    let mut res = 0;
    while let Some((i, j, x, y)) = queue.pop_front() {
        if ['p', 'B'].contains(&board[i as usize][j as usize]) {
            if board[i as usize][j as usize] == 'p' {
                res += 1;
            }
            continue;
        }

        if i+x < 0 || i+x == 8 || j+y < 0 || j+y == 8 {
            continue;
        }

        queue.push_back((i+x, j+y, x, y));
    }

    res
}

pub fn main() {
    let board = [
        [".",".",".",".",".",".",".","."],
        [".",".",".","p",".",".",".","."],
        [".",".",".","p",".",".",".","."],
        ["p","p",".","R",".","p","B","."],
        [".",".",".",".",".",".",".","."],
        [".",".",".","B",".",".",".","."],
        [".",".",".","p",".",".",".","."],
        [".",".",".",".",".",".",".","."]
    ].into_iter().map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect()).collect();
    println!("{}", num_rook_captures(board));
}
