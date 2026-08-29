use std::{cmp::Reverse, collections::BinaryHeap};

const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut queries = queries.into_iter().enumerate().map(|v| (v.0, v.1, 0)).collect::<Vec<_>>();
    queries.sort_by_key(|v| v.1);
    let (m, n, k) = (grid.len(), grid[0].len(), queries.len());

    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse((grid[0][0], 0, 0)));
    let mut idx = 0;
    let mut visited = vec![vec![false; n]; m];
    visited[0][0] = true;
    while idx < k && let Some(Reverse((num, i, j))) = min_heap.pop() {
        while idx < k && num >= queries[idx].1 {
            idx += 1;
        }
        if idx < k {
            queries[idx].2 += 1;
            for direct in DIRECTIONS {
                let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                if i < m && j < n && !visited[i][j] {
                    visited[i][j] = true;
                    min_heap.push(Reverse((grid[i][j], i, j)));
                }
            }
        }
    }

    let mut res = vec![0; k];
    for i in 0..k {
        if i>0 { queries[i].2 += queries[i-1].2 }
        res[queries[i].0] = queries[i].2;
    }
    res
}

pub fn main() {
    let grid = [[1,2,3],[2,5,7],[3,5,1]].into_iter().map(Vec::from).collect();
    let queries = [5,6,2].to_vec();
    println!("{:?}", max_points(grid, queries))
}
