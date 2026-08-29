fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let mut matrix = [[false; 8]; 8];
    for queen in queens {
        matrix[queen[0] as usize][queen[1] as usize] = true;
    }

    let mut res = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((king[0], king[1], 0, 1));
    queue.push_back((king[0], king[1], 0, -1));
    queue.push_back((king[0], king[1], 1, 0));
    queue.push_back((king[0], king[1], -1, 0));
    queue.push_back((king[0], king[1], -1, 1));
    queue.push_back((king[0], king[1], -1, -1));
    queue.push_back((king[0], king[1], 1, 1));
    queue.push_back((king[0], king[1], 1, -1));

    while let Some((i, j, x_direct, y_direct)) = queue.pop_front() {
        if i < 0 || i == 8 || j < 0 || j == 8 {
            continue;
        }
        if matrix[i as usize][j as usize] {
            res.push(vec![i, j]);
            continue;
        }

        queue.push_back((i+x_direct, j+y_direct, x_direct, y_direct));
    }

    res
}

pub fn main() {
    let queens = [[0,1],[1,0],[4,0],[0,4],[3,3],[2,4]].into_iter().map(Vec::from).collect();
    let king = [0,0].to_vec();
    println!("{:?}", queens_attackthe_king(queens, king));
}
