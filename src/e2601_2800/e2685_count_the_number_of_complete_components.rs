fn dfs(
    vertice: usize,
    hashmap: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    components: &mut Vec<usize>,
) {
    if visited[vertice] {
        return;
    }

    visited[vertice] = true;
    components.push(vertice);
    for vertice in hashmap[vertice].clone() {
        dfs(vertice, hashmap, visited, components);
    }
}

fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut hashmap = vec![vec![]; n];
    for vertices in edges {
        let (a, b) = (vertices[0] as usize, vertices[1] as usize);
        hashmap[a].push(b);
        hashmap[b].push(a);
    }

    let mut visited = vec![false; n];
    let mut res = 0;
    for vertice in 0..n {
        if !visited[vertice] {
            let mut components = Vec::new();
            dfs(vertice, &hashmap, &mut visited, &mut components);
            let n = components.len();
            if components.into_iter().all(|vertice| hashmap[vertice].len() == n-1) {
                res += 1;
            }
        }
    }

    res
}

pub fn main() {
    let n = 3;
    let edges = [[1,0], [2,1]].into_iter().map(Vec::from).collect();
    println!("{}", count_complete_components(n, edges));
}
