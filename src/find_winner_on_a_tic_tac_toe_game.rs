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
    let mut current = 'A';
    for vec in &moves {
        let (i, j) = (vec[0], vec[1]);
        hashset.insert((current, i, j));
        current = if current == 'A' { 'B' } else { 'A' };
    }

    for &value in &hashset {
        for directs in DIRECTIONS {
            if directs.iter().all(|v| hashset.contains(&(value.0, value.1 + v.0, value.2 + v.1))) {
                return value.0.to_string();
            }
        }
    }

    if moves.len() != 9 { "Pending".to_string() } else { "Draw".to_string() }
}

pub fn main() {
    let moves = [[0,0],[2,0],[1,1],[2,1],[2,2]].into_iter().map(Vec::from).collect();
    println!("{}", tictactoe(moves));
}
