fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            res.push(vec![i, j]);
        }
    }

    res.sort_by_key(|v| (r_center-v[0]).abs() + (c_center-v[1]).abs());
    res
}

pub fn main() {
    let rows = 1;
    let cols = 2;
    let r_center = 0;
    let c_center = 0;
    println!("{:?}", all_cells_dist_order(rows, cols, r_center, c_center));
}
