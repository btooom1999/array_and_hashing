fn maximum_score(scores: Vec<i32>, mut edges: Vec<Vec<i32>>) -> i32 {
    edges.sort_by_key(|v| scores[v[0] as usize] + scores[v[1] as usize]);
    let mut map = vec![std::collections::HashSet::new(); scores.len()];
    for edge in &edges {
        let (x, y) = (edge[0] as usize, edge[1] as usize);
        map[x].insert(y);
        map[y].insert(x);
    }


    let n = edges.len();
    let mut j = n.saturating_sub(1);
    let mut i = j-1;
    let mut min_i = 0;
    let mut res = -1;
    while j > 0 && j < n && i >= min_i {
        if scores[edges[i][0] as usize] + scores[edges[i][1] as usize] + scores[edges[j][0] as usize] + scores[edges[j][1] as usize] < res {
            break;
        };

        while i < n && i >= min_i {
            let (i1, i2) = (edges[i][0] as usize, edges[i][1] as usize);
            let (j1, j2) = (edges[j][0] as usize, edges[j][1] as usize);

            if scores[i1] + scores[i2] + scores[j1] + scores[j2] < res {
                break;
            };

            if (map[j1].contains(&i1) || map[j1].contains(&i2) || map[j2].contains(&i1) || map[j2].contains(&i2))
            && i1 != j1 && i1 != j2 && i2 != j1 && i2 != j2
            && scores[i1] + scores[i2] + scores[j1] + scores[j2] > res {
                res = scores[i1] + scores[i2] + scores[j1] + scores[j2];
                min_i = i+1;
                break;
            } else {
                i = i.wrapping_sub(1);
            }
        }

        j -= 1;
        i = j.wrapping_sub(1);
    }

    res
}

pub fn main() {
    let scores = [5,2,9,8,4].to_vec();
    let edges = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]].into_iter().map(Vec::from).collect();
    println!("{}", maximum_score(scores, edges));
}
