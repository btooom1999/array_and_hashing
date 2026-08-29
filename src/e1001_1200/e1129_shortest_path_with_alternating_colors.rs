use std::collections::VecDeque;

fn dfs(from: usize, to: usize, n: usize, map: &Vec<Vec<Vec<usize>>>) -> i32 {
    let mut queue = VecDeque::from([(from,0,true), (from,0,false)]);
    let mut visited = vec![vec![false; n]; 2];

    while let Some((dist, path, blue)) = queue.pop_front() {
        if visited[blue as usize][dist] {
            continue;
        }

        if dist == to {
            return path;
        }

        visited[blue as usize][dist] = true;
        for &dist in &map[(!blue) as usize][dist] {
            queue.push_back((dist, path+1, !blue));
        }
    }

    -1
}

fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut map = vec![vec![vec![]; n]; 2];
    for vertices in red_edges.into_iter() {
        map[0][vertices[0] as usize].push(vertices[1] as usize);
    }

    for vertices in blue_edges.into_iter() {
        map[1][vertices[0] as usize].push(vertices[1] as usize);
    }

    (0..n).map(|v| dfs(0, v, n, &map)).collect()
}

pub fn main() {
    let n = 3;
    let red_edges = [[0,1]].into_iter().map(Vec::from).collect();
    let blue_edges = [[2,1]].into_iter().map(Vec::from).collect();
    println!("{:?}", shortest_alternating_paths(n, red_edges, blue_edges));
}
