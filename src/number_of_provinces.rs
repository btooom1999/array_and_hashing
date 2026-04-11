fn dfs(
    city: usize,
    hashmap: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
) {
    if visited[city] {
        return;
    }

    visited[city] = true;
    for city in hashmap[city].clone() {
        dfs(city, hashmap, visited);
    }
}

fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut hashmap = vec![vec![]; n];
    for (i, cities) in is_connected.iter().enumerate() {
        for (j, &has_connected) in cities.iter().enumerate() {
            if i == j {
                continue;
            }

            if has_connected == 1 {
                hashmap[i].push(j);
            }
        }
    }

    let mut count = 0;
    let mut visited = vec![false; n+1];
    for i in 0..n {
        if !visited[i] {
            count += 1;
            dfs(i, &hashmap, &mut visited);
        }
    }

    count
}

pub fn main() {
    let is_connected = [[1,1,0],[1,1,0],[0,0,1]].into_iter().map(Vec::from).collect();
    println!("{}", find_circle_num(is_connected));
}
