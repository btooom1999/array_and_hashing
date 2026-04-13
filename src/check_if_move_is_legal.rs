fn dfs(
    direction: (i32, i32),
    origin: (i32, i32),
    max: (i32, i32),
    endpoint: char,
    board: &Vec<Vec<char>>,
    count: i32,
    mut other_color_count: i32,
) -> bool {
    if origin.0 < 0 || origin.1 < 0 || origin.0 == max.0 || origin.1 == max.1 {
        return false;
    }

    let val = board[origin.0 as usize][origin.1 as usize];
    if val == '.' {
        return false;
    }

    if val != endpoint {
        other_color_count += 1;
    }

    if count > 2 && val == endpoint && other_color_count + 2 == count {
        return true;
    }

    dfs(direction, (origin.0 + direction.0, origin.1 + direction.1), max, endpoint, board, count + 1, other_color_count)
}

fn check_move(mut board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
    board[r_move as usize][c_move as usize] = color;
    let (m, n) = (board.len() as i32, board[0].len() as i32);

    let mut res = false;
    for direct in [(0,1), (0,-1), (1,0), (-1,0), (1,1), (-1,-1), (-1,1), (1,-1)] {
        res = res || dfs(direct, (r_move, c_move), (m, n), color, &board, 1, 0);
    }

    res
}

pub fn main() {
    let board = [
        [".",".",".","W",".",".",".","."].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        [".",".",".","B",".",".",".","."].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        [".",".",".","B",".",".",".","."].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        [".",".",".","W",".",".",".","."].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        ["W","B","B",".","W","W","W","W"].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        [".",".",".","B",".",".",".","."].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        [".",".",".","B",".",".",".","."].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>(),
        [".",".",".","W",".",".",".","."].into_iter().map(|v| v.chars().next().unwrap()).collect::<Vec<_>>()
    ].to_vec();
    let r_move = 4;
    let c_move = 3;
    let color = 'B';
    println!("{}", check_move(board, r_move, c_move, color))
}
