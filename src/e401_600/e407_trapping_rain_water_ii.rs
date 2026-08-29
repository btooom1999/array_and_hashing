use std::{cmp::Reverse, collections::BinaryHeap};

const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (height_map.len(), height_map[0].len());
    let mut min_heap = BinaryHeap::new();
    let mut visited = vec![vec![false; n]; m];
    for j in 0..n {
        min_heap.push(Reverse((height_map[0][j], 0, j)));
        min_heap.push(Reverse((height_map[m-1][j], m-1, j)));
        visited[0][j] = true;
        visited[m-1][j] = true;
    }

    for i in 1..m-1 {
        min_heap.push(Reverse((height_map[i][0], i, 0)));
        visited[i][0] = true;
        min_heap.push(Reverse((height_map[i][n-1], i, n-1)));
        visited[i][n-1] = true;
    }


    let mut res = 0;
    while let Some(Reverse((height, i, j))) = min_heap.pop() {
        for direct in DIRECTIONS {
            let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if ni < m && nj < n && !visited[ni][nj] {
                if height_map[ni][nj] < height {
                    res += height - height_map[ni][nj];
                    height_map[ni][nj] = height;
                }

                min_heap.push(Reverse((height_map[ni][nj], ni, nj)));
                visited[ni][nj] = true;
            }
        }
    }

    res
}

pub fn main() {
    let height_map = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]].into_iter().map(Vec::from).collect();
    println!("{}", trap_rain_water(height_map));
}

