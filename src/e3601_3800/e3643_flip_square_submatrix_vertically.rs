fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
    let (x, y, k) = (x as usize, y as usize, k as usize);
    for i in 0..k/2 {
        for j in 0..k {
            (grid[x+i][y+j], grid[x+k-i-1][y+j]) = (grid[x+k-i-1][y+j], grid[x+i][y+j]);
        }
    }

    grid
}

pub fn main() {
    let grid = [[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]].into_iter().map(Vec::from).collect();
    let x = 1;
    let y = 0;
    let k = 3;
    println!("{:?}", reverse_submatrix(grid, x, y, k));
}
