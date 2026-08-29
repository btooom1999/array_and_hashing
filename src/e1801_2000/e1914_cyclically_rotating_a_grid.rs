fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let (mut m, mut n) = (grid.len(), grid[0].len());
    let turn = k as usize;
    let mut k = 0;
    while k != m && k != n {
        let mut list = Vec::new();
        for j in k..n { list.push((grid[k][j], k, j)); }
        list.pop();
        for i in k..m { list.push((grid[i][n-1], i, n-1)); }
        list.pop();
        for j  in (k..n).rev() { list.push((grid[m-1][j], m-1, j)); }
        list.pop();
        for i in (k..m).rev() { list.push((grid[i][k], i, k)); }
        list.pop();
        k += 1;
        n -= 1;
        m -= 1;

        let mut temp = list.clone();
        temp.rotate_left(turn % list.len());
        for i in 0..temp.len() {
            grid[list[i].1][list[i].2] = temp[i].0;
        }
    }

    grid
}

pub fn main() {
    let grid = [[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]].into_iter().map(Vec::from).collect();
    let k = 2;
    println!("{:?}", rotate_grid(grid, k));
}
