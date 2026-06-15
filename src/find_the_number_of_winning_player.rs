fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
    let mut hashmap = vec![[0; 11]; n as usize];
    for p in pick {
        hashmap[p[0] as usize][p[1] as usize] += 1;
    }

    hashmap.into_iter().enumerate().fold(0, |acc, (i, values)| {
        if values.into_iter().any(|sum| sum > i) {
            return acc + 1;
        }

        acc
    })
}

pub fn main() {
    let n = 5;
    // let pick = [[0,0],[1,0],[1,0],[2,1],[2,1],[2,0]].into_iter().map(Vec::from).collect();
    let pick = vec![vec![0,1], vec![0,1]];
    println!("{}", winning_player_count(n, pick));
}
