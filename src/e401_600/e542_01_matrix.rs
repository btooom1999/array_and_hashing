use std::collections::VecDeque;

fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (mat.len(), mat[0].len());

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 0 {
                mat[i][j] = 0;
                visited[i][j] = true;
                queue.push_back((i as i32, j as i32, 0));
            }
        }
    }

    while let Some((i, j, height)) = queue.pop_front() {
        for direct in [(0,1),(0,-1),(1,0),(-1,0)] {
            let ni = i + direct.0;
            let nj = j + direct.1;

            if ni < 0 || ni == m as i32 || nj < 0 || nj == n as i32 || visited[ni as usize][nj as usize] {
                continue;
            }

            mat[ni as usize][nj as usize] = height+1;
            visited[ni as usize][nj as usize] = true;
            queue.push_back((ni, nj, height+1));
        }
    }

    mat
}

pub fn main() {
    let mat = [[0,0,0],[0,1,0],[0,0,0]].into_iter().map(Vec::from).collect();
    println!("{:?}", update_matrix(mat));
}
