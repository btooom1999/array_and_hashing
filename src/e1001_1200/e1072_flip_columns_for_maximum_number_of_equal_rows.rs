fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let mut hashmap = std::collections::HashMap::new();
    for row in matrix {
        let flip = row.iter().map(|v| 1 - v).collect();
        *hashmap.entry(row).or_default() += 1;
        *hashmap.entry(flip).or_default() += 1;
    }

    *hashmap.values().max().unwrap()
}

pub fn main() {
    // let matrix = [[0,0,1,1],[1,0,1,0],[1,1,0,0]].into_iter().map(Vec::from).collect();
    let matrix = [[1,1],[1,1]].into_iter().map(Vec::from).collect();
    println!("{}", max_equal_rows_after_flips(matrix));
}
