fn backtracking(
    n: i32,
    i: i32,
    count: i32,
    visited: &mut i128,
    data: &mut Vec<Vec<char>>,
    result: &mut Vec<Vec<String>>,
) {
    if count == 0 {
        result.push(data.iter().cloned().map(|v| v.into_iter().collect::<_>()).collect());
        return;
    }

    if i < n {
        for j in 0..n {
            if *visited & 1 << (i*n+j) == 0 {
                data[i as usize][j as usize] = 'Q';
                *visited ^= 1 << (i*n+j);
                let mut queue = std::collections::VecDeque::from([
                    (i,j,0,1),
                    (i,j,0,-1),
                    (i,j,1,0),
                    (i,j,1,1),
                    (i,j,1,-1),
                ]);

                let mut last_queue = std::collections::VecDeque::new();
                while let Some((i, j, direct0, direct1)) = queue.pop_front() {
                    let i = i + direct0;
                    let j = j + direct1;
                    if i == n || j == n || j < 0 {
                        continue;
                    }

                    if *visited & 1 << (i*n+j) == 0 { last_queue.push_back((i, j)); }
                    *visited |= 1 << (i*n+j);
                    queue.push_back((i, j, direct0, direct1));
                }

                backtracking(n, i+1, count-1, visited, data, result);

                data[i as usize][j as usize] = '.';
                *visited ^= 1 << (i*n+j);
                while let Some((i, j)) = last_queue.pop_front() {
                    *visited ^= 1 << (i*n+j);
                }
            }
        }
    }

}

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    backtracking(n, 0, n, &mut 0, &mut vec![vec!['.'; n as usize]; n as usize], &mut result);
    result
}

pub fn main() {
    let n = 4;
    println!("{:?}", solve_n_queens(n));
}
