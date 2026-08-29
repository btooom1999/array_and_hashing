use std::collections::{HashMap, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    let grid = grid.into_iter().map(|v| v.into_bytes()).collect::<Vec<_>>();
    let (m, n) = (grid.len(), grid[0].len());
    let mut at = (0, 0);
    let mut total_mask = 0;
    let mut hashmap = HashMap::new();
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == b'@' {
                at = (i, j);
            } else if grid[i][j].is_ascii_lowercase() {
                total_mask ^= 1 << (grid[i][j] - b'a');
            }
        }
    }

    let res = i32::MAX;
    hashmap.insert((0, at.0, at.1), 0); // (key=(bit, index), value=step)
    while !hashmap.is_empty() {
        let mut temp_hashmap = HashMap::new();
        for (&(mask, i, j), &value) in &hashmap {
            if mask == total_mask {
                return value
            }

            let mut temp_grid = grid.clone();
            let mut queue = VecDeque::from([(i, j, 0)]);
            while let Some((i, j, step)) = queue.pop_front() {
                if grid[i][j].is_ascii_lowercase() && mask >> (grid[i][j] - b'a') & 1 == 0 {
                    let mask = mask ^ 1 << (grid[i][j] - b'a');
                    temp_hashmap.entry((mask, i, j)).and_modify(|v| *v = (value+step).min(*v)).or_insert(value + step);
                }

                for direct in DIRECTIONS {
                    let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                    let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                    if i < m && j < n && temp_grid[i][j] != b'#' {
                        if temp_grid[i][j].is_ascii_uppercase() && mask >> (temp_grid[i][j] - b'A') & 1 == 0 {
                            continue;
                        }
                        queue.push_back((i, j, step+1));
                        temp_grid[i][j] = b'#';
                    }
                }
            }
        }

        hashmap = temp_hashmap
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let grid =  ["@.a..","###.#","b.A.B"].into_iter().map(String::from).collect();
    // let grid = ["..#....##.","....d.#.D#","#...#.c...","..##.#..a.","...#....##","#....b....",".#..#.....","..........",".#..##..A.",".B..C.#..@"].into_iter().map(String::from).collect();
    println!("{}", shortest_path_all_keys(grid));
}
