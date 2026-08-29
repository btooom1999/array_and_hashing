const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (heights.len(), heights[0].len());
    let mut hashset = std::collections::HashSet::new();
    for i in 0..m {
        for j in 0..n {
            for direct in DIRECTIONS {
                let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                if ni < m && nj < n {
                    hashset.insert((heights[i][j]-heights[ni][nj]).abs());
                }
            }
        }
    }

    let mut hashset = hashset.into_iter().collect::<Vec<_>>();
    hashset.sort_unstable();

    let check = |k: i32| -> bool {
        let mut queue = std::collections::VecDeque::from([(0,0)]);
        let mut visited = vec![vec![false; n]; m];
        visited[0][0] = true;
        while let Some((i, j)) = queue.pop_front() {
            if i == m-1 && j == n-1 { return true; }
            for direct in DIRECTIONS {
                let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                if ni < m && nj < n && !visited[ni][nj] && (heights[i][j]-heights[ni][nj]).abs() <= k {
                    visited[ni][nj] = true;
                    queue.push_back((ni, nj));
                }
            }
        }

        false
    };

    let mut l = 0;
    let mut r = hashset.len();
    while l < r {
        let m = (l+r)/2;
        if check(hashset[m]) {
            r = m;
        } else {
            l = m+1;
        }
    }

    *hashset.get(r).unwrap_or(&0)
}

pub fn main() {
    // let heights = [[1,2,2],[3,8,2],[5,3,5]].into_iter().map(Vec::from).collect();
    // let heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]].into_iter().map(Vec::from).collect();
    let heights = vec![vec![3]];
    println!("{}", minimum_effort_path(heights));
}
