
const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    let mut hashset = std::collections::HashSet::new();
    hashset.insert(board.clone());
    for i in 0..2 {
        for j in 0..3 {
            if board[i][j] == 0 {
                let mut queue = std::collections::VecDeque::from([(board.clone(), i, j, 0)]);
                while let Some((mut board, i, j, step)) = queue.pop_front() {
                    if board == vec![vec![1,2,3], vec![4,5,0]] {
                        return step;
                    }

                    for direct in DIRECTIONS {
                        let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                        let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                        if ni < 2 && nj < 3 {
                            (board[i][j], board[ni][nj]) = (board[ni][nj], board[i][j]);
                            if !hashset.contains(&board) {
                                hashset.insert(board.clone());
                                queue.push_back((board.clone(), ni, nj, step+1));
                            }
                            (board[i][j], board[ni][nj]) = (board[ni][nj], board[i][j]);
                        }
                    }
                }

                return -1;
            }
        }
    }

    unreachable!()
}

pub fn main() {
    let board = [[1,2,3], [4,0,5]].into_iter().map(Vec::from).collect();
    println!("{}", sliding_puzzle(board));
}
