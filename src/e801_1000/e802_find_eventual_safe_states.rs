fn dfs(
    node: usize,
    map: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    dp: &mut Vec<i32>,
) -> bool {
    if visited[node] {
        return false;
    }

    if dp[node] != -1 {
        return dp[node] == 1;
    }

    if map[node].is_empty() {
        return true;
    }

    let mut is_valid = 1;
    visited[node] = true;
    for next_node in map[node].clone() {
        if !dfs(next_node, map, visited, dp) {
            is_valid = 0;
            break;
        }
    }

    visited[node] = false;
    dp[node] = is_valid;
    is_valid == 1
}

fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let n = graph.len();
    let mut map = vec![vec![]; graph.len()];
    for i in 0..n {
        for &node in &graph[i] {
            map[i].push(node as usize);
        }
    }

    let mut dp = vec![-1; n];
    let mut visited = vec![false; n];
    let mut res = Vec::new();
    for node in 0..n {
        if dfs(node, &map, &mut visited, &mut dp) {
            res.push(node as i32);
        }
    }

    res
}

pub fn main() {
    let graph = [
        [1,2].to_vec(),
        [2,3].to_vec(),
        [5].to_vec(),
        [0].to_vec(),
        [5].to_vec(),
        [].to_vec(),
        [].to_vec(),
    ].to_vec();
    println!("{:?}", eventual_safe_nodes(graph));
}
