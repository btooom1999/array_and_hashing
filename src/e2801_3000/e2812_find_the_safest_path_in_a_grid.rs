const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut thieves = std::collections::VecDeque::new();

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                grid[i][j] = 0;
                thieves.push_back((i, j, 1));
            } else {
                grid[i][j] = -1;
            }
        }
    }

    while let Some((i, j, distance)) = thieves.pop_front() {
        for direct in DIRECTIONS {
            let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if i < n && j < n && grid[i][j] == -1 {
                grid[i][j] = distance;
                thieves.push_back((i, j, distance+1));
            }
        }
    }

    let mut min = grid[0][0];
    let mut heap = std::collections::BinaryHeap::from([(grid[0][0], 0,0)]);
    grid[0][0] = -1;
    while let Some((distance, i, j)) = heap.pop() {
        min = min.min(distance);
        if i == n-1 && j == n-1 {
            return min;
        }

        for direct in DIRECTIONS {
            let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if i < n && j < n && grid[i][j] >= 0 {
                heap.push((grid[i][j], i, j));
                grid[i][j] = -1;
            }
        }
    }

    unreachable!()
}

pub fn main() {
    let grid = [[0,0,0,1],[0,0,0,0],[0,0,0,0],[1,0,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", maximum_safeness_factor(grid));
}
