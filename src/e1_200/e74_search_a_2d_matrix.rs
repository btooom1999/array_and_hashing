fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix[0].len();
    for m in &matrix {
        let mut l = 0;
        let mut h = n as i32 - 1;
        if target < m[l as usize] {
            return false;
        }
        if target > m[h as usize] {
            continue;
        }
        while l <= h {
            let x = (h + l) / 2;
            if m[x as usize] == target {
                return true;
            } else if m[x as usize] > target {
                h = x - 1;
            } else {
                l = x + 1;
            }
        }
    }

    false
}

pub fn main() {
    let matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]].into_iter().map(Vec::from).collect::<Vec<_>>();
    let target = 59;
    println!("{}", search_matrix(matrix, target));
}
