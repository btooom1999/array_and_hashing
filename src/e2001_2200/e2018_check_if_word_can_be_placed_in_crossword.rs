fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
    let word = word.as_bytes();
    let word_n = word.len();
    let (m, n) = (board.len(), board[0].len());
    for j in 0..n {
        let mut below = (0, true);
        let mut above = (0, true);
        for i in 0..m {
            if below.1 && (board[i][j] == ' ' || board[i][j].is_ascii_lowercase()) {
                if below.0 < word_n && (board[i][j] == ' ' || board[i][j] == word[below.0] as char) {
                    below.0 += 1;
                } else {
                    below.1 = false;
                }
            } else if board[i][j] == '#' {
                if below.0 == word_n && below.1 { return true; }
                else { below = (0, true) }
            }

            if above.1 && (board[m-i-1][j] == ' ' || board[m-i-1][j].is_ascii_lowercase()) {
                if above.0 < word_n && (board[m-i-1][j] == ' ' || board[m-i-1][j] == word[above.0] as char) {
                    above.0 += 1;
                } else {
                    above.1 = false;
                }
            } else if board[m-i-1][j] == '#' {
                if above.0 == word_n && above.1 { return true; }
                else { above = (0, true) }
            }
        }

        if (below.0 == word_n && below.1) || (above.0 == word_n && above.1) { return true; }
    }

    for i in 0..m {
        let mut left = (0, true);
        let mut right = (0, true);
        for j in 0..n {
            if left.1 && (board[i][j] == ' ' || board[i][j].is_ascii_lowercase()) {
                if left.0 < word_n && (board[i][j] == ' ' || board[i][j] == word[left.0] as char) {
                    left.0 += 1;
                } else {
                    left.1 = false;
                }
            } else if board[i][j] == '#' {
                if left.0 == word_n && left.1 { return true; }
                else { left = (0, true) }
            }

            if right.1 && (board[i][n-j-1] == ' ' || board[i][n-j-1].is_ascii_lowercase()) {
                if right.0 < word_n && (board[i][n-j-1] == ' ' || board[i][n-j-1] == word[right.0] as char) {
                    right.0 += 1;
                } else {
                    right.1 = false;
                }
            } else if board[i][n-j-1] == '#' {
                if right.0 == word_n && right.1 { return true; }
                else { right = (0, true) }
            }
        }

        if (left.0 == word_n && left.1) || (right.0 == word_n && right.1) { return true; }
    }

    false
}

pub fn main() {
    let board = [[" ", "#", "a"], [" ", "#", "c"], [" ", "#", "a"]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    // let board = [["#", " ", "#"], [" ", " ", "#"], ["#", "c", " "]]
    //     .into_iter()
    //     .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
    //     .collect();
    // let board = [["#", " ", "#"], [" ", " ", "#"], ["#", " ", "c"]]
    //     .into_iter()
    //     .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
    //     .collect();
    let word = "ac".to_string();
    println!("{}", place_word_in_crossword(board, word));
}
