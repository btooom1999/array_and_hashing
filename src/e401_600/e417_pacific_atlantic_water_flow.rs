use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0, 1), (0, -1)];

fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (heights.len(), heights[0].len());
    let mut coordinates = vec![vec![[false; 2]; n]; m];
    let mut queue = std::collections::VecDeque::new();
    for j in 0..n {
        coordinates[0][j][0] = true;
        coordinates[m-1][j][1] = true;
        queue.push_back((0, j, 0));
        queue.push_back((m-1, j, 1));
    }
    for i in 0..m {
        coordinates[i][0][0] = true;
        coordinates[i][n-1][1] = true;
        queue.push_back((i, 0, 0));
        queue.push_back((i, n-1, 1));
    }

    let mut res = HashSet::new();
    while let Some((i, j, idx)) = queue.pop_front() {
        if coordinates[i][j][0] && coordinates[i][j][1] {
            res.insert(vec![i as i32, j as i32]);
        }

        for direct in DIRECTIONS {
            let ni = i as i32 + direct.0;
            let nj = j as i32 + direct.1;
            if ni < 0 || nj < 0 { continue; }
            let ni = ni as usize;
            let nj = nj as usize;
            if ni == m || nj == n || coordinates[ni][nj][idx] || heights[ni][nj] < heights[i][j] { continue; }
            coordinates[ni][nj][idx] = true;
            queue.push_back((ni, nj, idx));
        }
    }

    res.into_iter().collect()
}

pub fn main() {
    let heights = [[1,2,2,3,5],[3,2,3,4,6],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]].into_iter().map(Vec::from).collect();
    println!("{:?}", pacific_atlantic(heights));
}
