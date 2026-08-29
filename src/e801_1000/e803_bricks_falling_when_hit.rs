const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

#[derive(Debug)]
struct UnionFind {
    parent: Vec<Option<usize>>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { parent: vec![None; n] }
    }

    fn remove(&mut self, x: usize) {
        self.parent[x] = None;
    }

    fn add(&mut self, x: usize) {
        if self.parent[x].is_some() { return; }
        self.parent[x] = Some(x);
    }

    fn find(&self, mut x: usize) -> Option<usize> {
        while self.parent[x].is_some_and(|v| v != x) {
            x = self.parent[x].unwrap();
        }

        self.parent[x]
    }

    fn merge(&mut self, x: usize, y: usize) {
        if let (Some(root_x), Some(root_y)) = (Self::find(self, x), Self::find(self, y)) {
            if root_x == root_y { return; }

            self.parent[y] = Some(x);
        }
    }
}

fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut hashset = hits.iter().cloned().collect::<std::collections::HashSet<_>>();

    for j in 0..n {
        if grid[0][j] == 1 && !hashset.contains(&vec![0, j as i32]) {
            grid[0][j] = -1;
            let mut queue = std::collections::VecDeque::from([(0, j)]);
            while let Some((i, j)) = queue.pop_front() {
                for direct in DIRECTIONS {
                    let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                    let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                    if i < m && j < n && grid[i][j] == 1 && !hashset.contains(&vec![i as i32, j as i32]) {
                        grid[i][j] = -1;
                        queue.push_back((i, j));
                    }
                }
            }
        }
    }

    let mut res = Vec::new();
    for hit in hits.into_iter().rev() {
        let (i, j) = (hit[0] as usize, hit[1] as usize);
        if grid[i][j] == 1 && (i == 0 || DIRECTIONS.iter().any(|direct| {
            let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
            let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
            if i < m && j < n && grid[i][j] == -1 { return true; }
            false
        })) {
            grid[i][j] = -1;
        }

        let mut count = 0;
        if grid[i][j] == -1 {
            let mut queue = std::collections::VecDeque::from([(i, j)]);
            while let Some((i, j)) = queue.pop_front() {
                for direct in DIRECTIONS {
                    let i = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
                    let j = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
                    if i < m && j < n && grid[i][j] == 1 && !hashset.contains(&vec![i as i32, j as i32]) {
                        count += 1;
                        grid[i][j] = -1;
                        queue.push_back((i, j));
                    }
                }
            }
        }

        res.push(count);
        hashset.remove(&hit);
    }

    res.reverse();
    res
}

pub fn main() {
    let grid = [[1,0,0,0],[1,1,1,0]].into_iter().map(Vec::from).collect();
    let hits = [[1,0]].into_iter().map(Vec::from).collect();
    println!("{:?}", hit_bricks(grid, hits));
}
