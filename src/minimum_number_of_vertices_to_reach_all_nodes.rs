use std::collections::HashSet;

fn dfs(
    vertice: usize,
    hashmap: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    result: &mut Vec<i32>,
) {
    if visited[vertice] {
        return;
    }

    visited[vertice] = true;
    for vertice in hashmap[vertice].clone() {
        result[vertice] += 1;
        dfs(vertice, hashmap, visited, result);
    }
}

fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![0; n as usize];
    for vertices in edges {
        result[vertices[1] as usize] += 1;
    }

    result
        .into_iter()
        .enumerate()
        .filter_map(|(i, count)| {
            if count > 0 {
                return None;
            }

            Some(i as i32)
        }).collect()
}

pub fn main() {
    let n = 6;
    let edges = [[0,1],[0,2],[2,5],[3,4],[4,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_smallest_set_of_vertices(n, edges));
}
