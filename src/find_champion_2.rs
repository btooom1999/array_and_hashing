fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut result = vec![true; n as usize];

    for team in edges {
        result[team[1] as usize] = false;
    }

    let mut res = -1;
    for i in 0..result.len() {
        if result[i] {
            if res > -1 {
                return -1;
            }

            res = i as i32;
        }
    }

    res
}

pub fn main() {
    let n = 3;
    let edges = [[0,1],[1,2]].into_iter().map(Vec::from).collect();
    println!("{}", find_champion(n, edges));
}
