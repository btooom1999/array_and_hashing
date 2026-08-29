const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut key = -1;
    let mut islands = Vec::new();
    islands.push((0, 0));

    let mut max = 0;
    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                grid[i][j] = key;
                let mut count = 1;
                let mut queue = std::collections::VecDeque::from([(i, j)]);
                while let Some((i, j)) = queue.pop_front() {
                    for direct in DIRECTIONS {
                        let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                        let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                        if i < n && j < n && grid[i][j] == 1 {
                            count += 1;
                            grid[i][j] = key;
                            queue.push_back((i, j));
                        }
                    }
                }

                max = max.max(count);
                islands.push((key, count));
                key -= 1;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 0 {
                let mut total = 0;
                let mut visited = vec![false; islands.len()];
                for direct in DIRECTIONS {
                    let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                    let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                    if i < n && j < n && grid[i][j] != 0 {
                        let k = grid[i][j].unsigned_abs() as usize;
                        if !visited[k] {
                            total += islands[k].1;
                            visited[k] = true;
                            max = max.max(total + 1);
                        }
                    }
                }
            }
        }
    }

    max
}

pub fn main() {
    let grid = [[1,0],[0,1]].into_iter().map(Vec::from).collect();
    println!("{}", largest_island(grid));
}
