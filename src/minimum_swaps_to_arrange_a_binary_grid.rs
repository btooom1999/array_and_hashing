fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut counts = vec![0; n];
    for (i, row) in grid.into_iter().enumerate() {
        for num in row.into_iter().rev() {
            if num == 1 { break; }
            counts[i] += 1;
        }
    }

    let mut res = 0;
    for i in 0..n {
        if counts[i] < n-i-1 {
            if let Some(k) = (i+1..n).find(|&k| counts[k] >= n-i-1) {
                res += k-i;
                for k in (i..k).rev() {
                    (counts[k], counts[k+1]) = (counts[k+1], counts[k]);
                }
            } else {
                return -1;
            }
        }
    }

    res as i32
}

pub fn main() {
    let grid = [[0,0,1],[1,1,0],[1,0,0]].into_iter().map(Vec::from).collect();
    println!("{}", min_swaps(grid));
}
