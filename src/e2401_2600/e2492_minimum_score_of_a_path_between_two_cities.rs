use std::collections::VecDeque;

fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut map = vec![vec![]; n+1];
    for road in roads {
        let (a, b, dist) = (road[0] as usize, road[1] as usize, road[2]);
        map[a].push((b, dist));
        map[b].push((a, dist));
    }

    let mut res = i32::MAX;
    let mut queue = VecDeque::from([(1, i32::MAX)]);

    let mut visited = vec![false; n+1];
    while let Some((path, dist)) = queue.pop_front() {
        res = res.min(dist);

        if visited[path] {
            continue;
        }

        visited[path] = true;
        for &(path, dist) in &map[path] {
            queue.push_back((path, dist));
        }
    }

    res
}

pub fn main() {
    let n = 4;
    let roads = [[1,2,9],[2,3,6],[2,4,5],[1,4,7]].into_iter().map(Vec::from).collect();
    println!("{}", min_score(n, roads));
}
