fn bob_dfs(
    node: usize,
    traversed_path: &mut Vec<i32>,
    step: i32,
    map: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
) -> bool {
    if node == 0 {
        traversed_path[0] = step;
        return true;
    }

    visited[node] = true;
    for &next_node in &map[node] {
        if visited[next_node] {
            continue;
        }

        if bob_dfs(next_node, traversed_path, step+1, map, visited) {
            traversed_path[node] = step;
            return true;
        }
    }

    false
}

fn alice_dfs(
    node: usize,
    visited: &mut Vec<bool>,
    bob_traversed_path: &Vec<i32>,
    map: &Vec<Vec<usize>>,
    amount: &Vec<i32>,
    step: i32,
    mut sum: i32,
) -> i32 {
    let price = amount[node];
    let bob_step = bob_traversed_path[node];
    if bob_step == -1 || step < bob_step {
        sum += price;
    } else if step == bob_step {
        sum += price / 2;
    }

    if map[node].len() == 1 && node != 0 {
        return sum;
    }

    visited[node] = true;
    let mut max = i32::MIN;
    for &node in &map[node] {
        if visited[node] {
            continue;
        }

        max = max.max(alice_dfs(node, visited, bob_traversed_path, map, amount, step+1, sum));
    }

    max
}

fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
    let n = edges.len()+1;
    let mut map = vec![vec![]; n];
    for vertices in edges.iter() {
        let (a, b) = (vertices[0] as usize, vertices[1] as usize);
        map[a].push(b);
        map[b].push(a);
    }

    let mut bob_traversed_path = vec![-1; n];
    bob_dfs(bob as usize, &mut bob_traversed_path, 0, &map, &mut vec![false; n]);
    alice_dfs(0, &mut vec![false; n], &bob_traversed_path, &map, &amount, 0, 0)
}

pub fn main() {
    let edges = [[0,1],[1,2],[1,3],[3,4]].into_iter().map(Vec::from).collect();
    let bob = 3;
    let amount = [-2,4,2,-4,6].to_vec();
    println!("{}", most_profitable_path(edges, bob, amount));
}
