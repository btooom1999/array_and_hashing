struct UnionFind {
    parent: Vec<Option<usize>>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { parent: vec![None; n], rank: vec![0; n] }
    }

    fn add(&mut self, x: usize) {
        if self.parent[x].is_some() { return; }
        self.parent[x] = Some(x);
    }

    fn find(&mut self, x: usize) -> Option<usize> {
        if self.parent[x].is_none_or(|v| v == x) { return self.parent[x]; }

        self.parent[x] = Self::find(self, self.parent[x].unwrap());
        self.parent[x]
    }

    fn merge(&mut self, x: usize, y: usize) {
        if let (Some(x), Some(y)) = (Self::find(self, x), Self::find(self, y)) && x != y {
            if self.rank[y] > self.rank[x] {
                self.rank.swap(x, y);
            }

            self.parent[y] = self.parent[x];
            if self.rank[y] == self.rank[x] {
                self.rank[x] += 1;
            }
        }
    }
}

fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (row as usize, col as usize);

    let mut dsu = UnionFind::new(m * (n+1));
    let coord = |r: usize, c: usize| -> usize { r * (n+1) + c };

    let first_row = coord(0, n);
    let last_row = coord(m-1, n);

    dsu.add(first_row);
    dsu.add(last_row);

    for (day, cell) in cells.iter().enumerate().rev() {
        let (r, c) = (cell[0]-1, cell[1]-1);

        let x = coord(r as usize, c as usize);
        dsu.add(x);

        for (nr, nc) in [(r+1, c), (r-1, c), (r, c+1), (r, c-1)] {
            if (0..m as i32).contains(&nr) && (0..n as i32).contains(&nc) {
                let y = coord(nr as usize, nc as usize);
                if let Some(y) = dsu.find(y) {
                    dsu.merge(x, y);
                }
            }

        }

        if r == 0 { dsu.merge(x, first_row); }
        else if r == (m-1) as i32 { dsu.merge(x, last_row); }

        if dsu.find(last_row) == dsu.find(first_row) {
            return day as i32;
        }
    }

    0
}

pub fn main() {
    let row = 2;
    let col = 2;
    let cells = [[1,1],[2,1],[1,2],[2,2]].into_iter().map(Vec::from).collect();
    println!("{}", latest_day_to_cross(row, col, cells));
}
