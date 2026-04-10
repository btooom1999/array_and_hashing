fn dfs(
    node: usize,
    visited: &mut Vec<bool>,
    hashmap: &Vec<Vec<usize>>,
) -> i32 {
    visited[node] = true;
    for node in hashmap[node].clone() {
        if visited[node] {
            continue;
        }

        dfs(node, visited, hashmap);
    }

    1
}

fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut hashmap = vec![vec![]; n];
    for points in edges {
        let (a, b) = (points[0] as usize, points[1] as usize);
        hashmap[a].push(b);
        hashmap[b].push(a);
    }

    let mut res = 0;
    let mut visited = vec![false; n];
    for node in 0..n {
        if !visited[node] {
            res += dfs(node, &mut visited, &hashmap)
        }
    }

    res
}

pub fn main() {
    let n = 5;
    let edges = [[0,1],[1,2],[3,4]].into_iter().map(Vec::from).collect();
    println!("{}", count_components(n, edges));
}
