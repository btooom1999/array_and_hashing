fn dfs(
    edge: usize,
    parent: i32,
    map: &mut Vec<Vec<i32>>,
    visited: &mut Vec<bool>,
    count: &mut i32,
) -> bool {
    *count += 1;

    if visited[edge] {
        return false;
    }

    if map[edge].is_empty() {
        return true;
    }

    visited[edge] = true;

    for next_edge in map[edge].clone() {
        if next_edge == parent {
            continue;
        }

        if !dfs(next_edge as usize, edge as i32, map, visited, count) {
            return false;
        }
    }

    visited[edge] = false;
    map[edge].clear();
    true
}

fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    let mut map = vec![vec![]; n];
    for item in edges {
        let (a, b) = (item[0], item[1]);
        map[a as usize].push(b);
        map[b as usize].push(a);
    }

    let mut visited = vec![false; n];
    let mut max = 0;
    for edge in 0..n {
        let mut count = 0;
        if !dfs(edge, -1, &mut map, &mut visited, &mut count) {
            return false;
        }

        max = max.max(count);
    }

    max == n as i32
}

pub fn main() {
    let n = 5;
    let edges = [[0, 1], [0, 2], [0, 3], [1, 4]].into_iter().map(Vec::from).collect();
    // let n = 5;
    // let edges = [[0,1],[2,0],[3,0],[1,4]].into_iter().map(Vec::from).collect();
    println!("{}", valid_tree(n, edges));
}
