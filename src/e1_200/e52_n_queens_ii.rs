fn backtracking(
    n: i32,
    i: i32,
    count: i32,
    visited: &mut i128,
) -> i32 {
    if count == 0 {
        return 1;
    }

    let mut res = 0;
    if i < n {
        for j in 0..n {
            if *visited & 1 << (i*n+j) == 0 {
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

                res += backtracking(n, i+1, count-1, visited);

                *visited ^= 1 << (i*n+j);
                while let Some((i, j)) = last_queue.pop_front() {
                    *visited ^= 1 << (i*n+j);
                }
            }
        }
    }

    res
}

fn total_n_queens(n: i32) -> i32 {
    backtracking(n, 0, n, &mut 0)
}

pub fn main() {
    let n = 4;
    println!("{}", total_n_queens(n));
}
