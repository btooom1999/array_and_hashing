const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (is_infected.len(), is_infected[0].len());
    let mut res = 0;

    loop {
        let mut visited = vec![vec![false; n]; m];
        let mut ones = false;
        let mut zeroes = false;
        let mut max = (0,0,0,0); // (i, j, max, walls);
        for i in 0..m {
            for j in 0..n {
                if is_infected[i][j] == 1 {
                    visited[i][j] = true;
                    let mut walls = 0;
                    let mut count = std::collections::HashSet::new();
                    let mut queue = std::collections::VecDeque::from([(i, j)]);
                    while let Some((i, j)) = queue.pop_front() {
                        for direct in DIRECTIONS {
                            let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                            let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                            if i < m && j < n && !visited[i][j] {
                                if is_infected[i][j] == 1 {
                                    ones = true;
                                    visited[i][j] = true;
                                    queue.push_back((i, j));
                                } else if is_infected[i][j] == 0 {
                                    zeroes = true;
                                    count.insert((i,j));
                                    walls += 1;
                                }
                            }
                        }
                    }

                    if count.len() as i32 > max.2 || (count.len() as i32 == max.2 && walls < max.2) {
                        max = (i, j, count.len() as i32, walls);
                    }
                }
            }
        }

        if !ones || !zeroes {
            return res;
        }

        res += max.3;
        let mut queue = std::collections::VecDeque::from([(max.0, max.1)]);
        visited[max.0][max.1] = false;
        is_infected[max.0][max.1] = -1;
        while let Some((i, j)) = queue.pop_front() {
            for direct in DIRECTIONS {
                let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                if i < m && j < n && visited[i][j] {
                    visited[i][j] = false;
                    queue.push_back((i, j));
                    is_infected[i][j] = -1;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if visited[i][j] {
                    visited[i][j] = false;
                    let mut queue = std::collections::VecDeque::from([(i, j)]);
                    while let Some((i, j)) = queue.pop_front() {
                        for direct in DIRECTIONS {
                            let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                            let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                            if i < m && j < n {
                                if visited[i][j] && is_infected[i][j] == 1 {
                                    queue.push_back((i, j));
                                    visited[i][j] = false;
                                } else if is_infected[i][j] == 0 {
                                    is_infected[i][j] = 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn main() {
    // let is_infected = [[0,1,0,0,0,0,0,1],[0,1,0,0,0,0,0,1],[0,0,0,0,0,0,0,1],[0,0,0,0,0,0,0,0]].into_iter().map(Vec::from).collect();
    // let is_infected = [[0,1,0,1,1,1,1,1,1,0],[0,0,0,1,0,0,0,0,0,0],[0,0,1,1,1,0,0,0,1,0],[0,0,0,1,1,0,0,1,1,0],[0,1,0,0,1,0,1,1,0,1],[0,0,0,1,0,1,0,1,1,1],[0,1,0,0,1,0,0,1,1,0],[0,1,0,1,0,0,0,1,1,0],[0,1,1,0,0,1,1,0,0,1],[1,0,1,1,0,1,0,1,0,1]].into_iter().map(Vec::from).collect();
    let is_infected = [[1,1,0,0,1,0,1,1,1,1,1,0,1,1,1,0,1,1,0,0],[1,1,1,1,1,1,1,1,0,1,0,0,0,0,0,1,0,1,0,0],[1,1,1,1,1,1,1,1,1,0,0,1,0,0,0,1,1,1,1,1],[1,1,0,1,1,0,1,0,1,1,0,0,0,0,0,1,1,1,0,1],[1,1,1,0,1,1,0,1,1,0,0,1,1,0,1,1,1,0,0,1],[0,1,0,1,0,1,0,1,0,0,0,0,1,1,1,0,1,0,1,0],[1,0,1,1,1,0,0,0,1,1,0,1,1,0,1,1,1,0,1,1],[1,0,0,1,1,1,0,0,1,1,1,1,0,1,1,1,0,1,0,0],[1,0,1,1,1,1,0,1,1,1,1,0,1,0,1,0,1,0,1,1],[1,0,0,1,1,1,1,1,1,0,1,1,1,0,1,1,0,1,1,1],[1,0,1,0,0,1,1,1,0,1,1,1,1,0,0,1,1,1,0,1],[1,0,1,1,1,0,1,1,1,1,0,1,0,0,1,1,0,1,1,1],[1,0,1,0,1,0,0,1,0,1,1,1,0,1,0,0,1,1,0,1],[1,1,0,0,0,1,0,0,1,1,1,0,0,0,0,1,0,1,0,1],[0,1,1,0,0,1,1,0,0,0,1,1,1,1,0,0,0,1,0,0],[1,1,1,1,1,1,0,1,0,0,1,0,1,1,1,1,0,0,0,0],[0,1,0,0,0,1,1,0,0,1,1,1,1,1,1,0,1,0,0,1],[1,1,1,0,1,1,0,1,0,1,1,1,0,0,1,1,1,1,0,1],[0,0,1,1,1,1,1,1,0,1,0,0,1,0,0,0,0,1,1,1],[0,1,1,1,1,0,1,0,1,1,1,1,0,0,0,1,0,0,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", contain_virus(is_infected));
}
