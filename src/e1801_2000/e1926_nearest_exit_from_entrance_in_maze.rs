const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let (m, n) = (maze.len(), maze[0].len());
    let (x, y) = (entrance[0] as usize, entrance[1] as usize);
    maze[x][y] = '+';
    let mut queue = std::collections::VecDeque::from([(x, y, 0)]);
    while let Some((i, j, count)) = queue.pop_front() {
        if (i != x || j != y) && (i == 0 || j == 0 || i == m-1 || j == n-1) {
            return count;
        }

        for direct in DIRECTIONS {
            let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if ni < m && nj < n && maze[ni][nj] == '.' {
                maze[ni][nj] = '+';
                queue.push_back((ni, nj, count+1));
            }
        }
    }

    -1
}

pub fn main() {
    let maze = [["+","+",".","+"],[".",".",".","+"],["+","+","+","."]]
        .into_iter()
        .map(|v| v.into_iter().map(|v| v.chars().next().unwrap()).collect())
        .collect();
    let entrance = [1,2].to_vec();
    println!("{}", nearest_exit(maze, entrance));
}
