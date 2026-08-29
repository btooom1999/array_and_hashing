use std::{cmp::Reverse, collections::BinaryHeap};

const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if n == 1 { return grid[0][0] };
    let mut heap = BinaryHeap::from([Reverse((grid[0][0], 0, 0))]);
    let mut visited = vec![vec![false; n]; n];
    visited[0][0] = true;

    let mut time = grid[0][0];
    while let Some(Reverse((max, i, j))) = heap.pop() {
        time = time.max(max);
        if i == n-1 && j == n-1 {
            break;
        }

        for direct in DIRECTIONS {
            let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if ni < n && nj < n && !visited[ni][nj] {
                heap.push(Reverse((grid[ni][nj], ni, nj)));
                visited[ni][nj] = true;
            }
        }
    }

    time
}

pub fn main() {
    let grid = [[0,2],[1,3]].into_iter().map(Vec::from).collect();
    println!("{}", swim_in_water(grid));
}
