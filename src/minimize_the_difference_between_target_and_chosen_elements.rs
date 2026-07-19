fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
    let mut hashset = mat[0].iter().cloned().collect::<std::collections::HashSet<_>>();
    for i in 1..mat.len() {
        let mut temp = std::collections::HashSet::new();
        let mut min = i32::MAX;
        for &num1 in &mat[i] {
            for &num2 in &hashset {
                if num1 + num2 <= target {
                    temp.insert(num1 + num2);
                } else {
                    min = min.min(num1 + num2);
                }
            }
        }

        if min != i32::MAX {
            temp.insert(min);
        }
        hashset = temp;
    }

    let mut res = i32::MAX;
    for num in hashset {
        res = res.min((target-num).abs());
    }

    res
}

pub fn main() {
    let mat = [[1,2,3],[4,5,6],[7,8,9]].into_iter().map(Vec::from).collect();
    let target = 13;
    println!("{}", minimize_the_difference(mat, target));
}
