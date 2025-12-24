use std::collections::HashMap;

fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut hashmap = HashMap::<i32, i32>::with_capacity(grid.len() * grid.len());

    for matrix in &grid {
        for val in matrix {
            *hashmap.entry(*val).or_default() += 1;
        }
    }

    let mut res = Vec::new();
    let mut count = 0;
    for matrix in &grid {
        for val in matrix {
            count += 1;
            if let Some(num) = hashmap.get(val) {
                if *num > 1 && !res.contains(val) {
                    res.insert(0, *val);
                }
                if !hashmap.contains_key(&count) {
                    res.push(count);
                }
            }
        }
    }

    res
}

pub fn main() {
    let grid = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
    println!("{:?}", find_missing_and_repeated_values(grid));
}
