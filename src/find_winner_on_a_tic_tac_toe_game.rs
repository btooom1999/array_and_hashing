const DIRECTIONS: [[(i32, i32); 2]; 8] = [
    [(0,1), (0,2)],
    [(0,-1), (0,-2)],
    [(1,0), (-1,0)],
    [(2,0), (-2,0)],
    [(1,1), (2,2)],
    [(-1,-1), (-2,-2)],
    [(-1,1), (-2,2)],
    [(1,-1), (2,-2)],
];

fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let mut hashset = std::collections::HashSet::new();
    let n = moves.len();
    for i in 0..n {
        hashset.insert((if i % 2 == 0 { 'A' } else { 'B' }, moves[i][0], moves[i][1]));
    }

    for &value in &hashset {
        for directs in DIRECTIONS {
            if directs.iter().all(|v| hashset.contains(&(value.0, value.1 + v.0, value.2 + v.1))) {
                return value.0.to_string();
            }
        }
    }

    if n != 9 { "Pending".to_string() } else { "Draw".to_string() }
}

pub fn main() {
    let moves = [[0,0],[2,0],[1,1],[2,1],[2,2]].into_iter().map(Vec::from).collect();
    println!("{}", tictactoe(moves));
}
