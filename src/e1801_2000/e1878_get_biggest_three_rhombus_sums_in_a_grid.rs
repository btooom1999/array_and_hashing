fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = std::collections::HashSet::new();
    let (m, n) = (grid.len(), grid[0].len());
    let mut prefix = vec![vec![(0,0); n]; m];
    for i in 0..m {
        for j in 0..n {
            prefix[i][j] = (grid[i][j], grid[i][j]);
            if i>0 && j>0 {
                prefix[i][j].1 += prefix[i-1][j-1].1;
            }
            if i>0 && j+1<n {
                prefix[i][j].0 += prefix[i-1][j+1].0;
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            res.insert(grid[i][j]);
            let mut k = 1;
            while j >= k && j+k < n && i+2*k < m {
                let a = prefix[i+k][j-k].0 - if i>0 && j+1<n { prefix[i-1][j+1].0 } else { 0 };
                let b = prefix[i+k][j+k].1 - if i>0 && j>0 { prefix[i-1][j-1].1 } else { 0 };
                let c = prefix[i+2*k][j].0 - if i+k>0 && j+k+1<n { prefix[i+k-1][j+k+1].0 } else { 0 };
                let d = prefix[i+2*k][j].1 - if i+k-1<m && j>k { prefix[i+k-1][j-k-1].1 } else { 0 };
                let val = a + b + c + d - grid[i][j] - grid[i+k][j-k] - grid[i+k][j+k] - grid[i+2*k][j];
                res.insert(val);
                k += 1;
            }
        }
    }

    let mut res = res.into_iter().collect::<Vec<_>>();
    res.sort_by(|a, b| b.cmp(a));
    res[..3.min(res.len())].to_vec()
}

pub fn main() {
    // let grid = [[3,4,5,1,3],[3,3,4,2,3],[20,30,200,40,10],[1,5,5,4,1],[4,3,2,2,5]].into_iter().map(Vec::from).collect();
    let grid = [[20,17,9,13,5,2,9,1,5],[14,9,9,9,16,18,3,4,12],[18,15,10,20,19,20,15,12,11],[19,16,19,18,8,13,15,14,11],[4,19,5,2,19,17,7,2,2]].into_iter().map(Vec::from).collect();
    println!("{:?}", get_biggest_three(grid));
}
