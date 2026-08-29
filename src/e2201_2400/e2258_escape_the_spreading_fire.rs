const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn maximum_minutes(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut queue = std::collections::VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                queue.push_back((i, j, 10));
                grid[i][j] = 10;
            }
        }
    }

    while let Some((i, j, num)) = queue.pop_front() {
        for direct in DIRECTIONS {
            let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if i < m && j < n && grid[i][j] == 0 {
                grid[i][j] = num+1;
                queue.push_back((i, j, num+1));
            }
        }
    }

    let check = |minute: i32| -> bool {
        let mut queue = std::collections::VecDeque::from([(0,0,0)]);
        let mut visited = vec![vec![false; n]; m];
        visited[0][0] = true;
        while let Some((i, j, step)) = queue.pop_front() {
            if i == m-1 && j == n-1 { return true; }
            for direct in DIRECTIONS {
                let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                if i < m && j < n && !visited[i][j] && (grid[i][j] == 0 || grid[i][j] >= 10) {
                    if grid[i][j] >= 10 && grid[i][j]-10 <= minute+step+1 && !(grid[i][j]-10 == minute+step+1 && i == m-1 && j == n-1) {
                        continue;
                    }

                    visited[i][j] = true;
                    queue.push_back((i, j, step+1));
                }
            }
        }

        false
    };

    let mut l = 0;
    let mut r = 1_000_000_000;
    while l <= r {
        let m = (l+r)/2;
        if check(m) {
            l = m+1;
        } else {
            r = m-1;
        }

    }

    r
}

pub fn main() {
    let grid = [[0,2,0,0,0,0,0],[0,0,0,2,2,1,0],[0,2,0,0,1,2,0],[0,0,2,2,2,0,2],[0,0,0,0,0,0,0]].into_iter().map(Vec::from).collect();
    // let grid = [[0,2,0,0,1],[0,2,0,2,2],[0,2,0,0,0],[0,0,2,2,0],[0,0,0,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", maximum_minutes(grid));
}

