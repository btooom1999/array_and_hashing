use std::{collections::BTreeSet, ops::Bound};

pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut res = i32::MIN;

    for left in 0..n {
        let mut sum_rows = vec![0; m];
        for right in left..n {
            for i in 0..m {
                sum_rows[i] += matrix[i][right];
            }

            let mut btreeset = BTreeSet::from([0]);
            let mut current_sum = 0;
            for &sum in &sum_rows {
                current_sum += sum;
                if let Some(&num) = btreeset.range((Bound::Included(current_sum-k), Bound::Unbounded)).next() {
                    res = res.max(current_sum - num);
                }

                btreeset.insert(current_sum);
            }
        }
    }

    res
}

pub fn main() {
    // let matrix = [[1,0,1], [0,-2,3]].into_iter().map(Vec::from).collect();
    let matrix = [[27,5,-20,-9,1,26,1,12,7,-4,8,7,-1,5,8],[16,28,8,3,16,28,-10,-7,-5,-13,7,9,20,-9,26],[24,-14,20,23,25,-16,-15,8,8,-6,-14,-6,12,-19,-13],[28,13,-17,20,-3,-18,12,5,1,25,25,-14,22,17,12],[7,29,-12,5,-5,26,-5,10,-5,24,-9,-19,20,0,18],[-7,-11,-8,12,19,18,-15,17,7,-1,-11,-10,-1,25,17],[-3,-20,-20,-7,14,-12,22,1,-9,11,14,-16,-5,-12,14],[-20,-4,-17,3,3,-18,22,-13,-1,16,-11,29,17,-2,22],[23,-15,24,26,28,-13,10,18,-6,29,27,-19,-19,-8,0],[5,9,23,11,-4,-20,18,29,-6,-4,-11,21,-6,24,12],[13,16,0,-20,22,21,26,-3,15,14,26,17,19,20,-5],[15,1,22,-6,1,-9,0,21,12,27,5,8,8,18,-1],[15,29,13,6,-11,7,-6,27,22,18,22,-3,-9,20,14],[26,-6,12,-10,0,26,10,1,11,-10,-16,-18,29,8,-8],[-19,14,15,18,-10,24,-9,-7,-19,-14,23,23,17,-5,6]].into_iter().map(Vec::from).collect();
    let k = -100;
    println!("{}", max_sum_submatrix(matrix, k))
}
