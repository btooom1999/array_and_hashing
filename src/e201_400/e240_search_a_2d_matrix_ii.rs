fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    for k in 0..m {
        if matrix[k][0] > target {
            return false;
        }
        if matrix[k].binary_search(&target).is_ok() {
            return true;
        }
    }

    false
}

pub fn main() {
    let matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]].into_iter().map(Vec::from).collect();
    let target = 21;
    println!("{}", search_matrix(matrix, target))
}
