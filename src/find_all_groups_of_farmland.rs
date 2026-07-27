fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (land.len(), land[0].len());
    let mut res = Vec::new();
    let mut visited = Vec::new();
    for i in 0..m {
        for j in 0..n {
            if land[i][j] == 1 && visited.iter().rev().all(|&(r1, c1, r2, c2)| !(r1 <= i && i <= r2 && c1 <= j && j <= c2)) {
                let mut y = 0;
                for k in i+1..m {
                    if land[k][j] == 0 { break };
                    y += 1;
                }
                let mut x = 0;
                for k in j+1..n {
                    if land[i][k] == 0 { break };
                    x += 1;
                }
                res.push(vec![i as i32, j as i32, (i+y) as i32, (j+x) as i32]);
                visited.push((i, j, i+y, j+x));
            }
        }
    }

    res
}

pub fn main() {
    let land = [[1,0,0],[0,1,1],[0,1,1]].into_iter().map(Vec::from).collect();
    println!("{:?}", find_farmland(land));
}
