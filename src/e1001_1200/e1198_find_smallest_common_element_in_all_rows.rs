use std::collections::HashMap;

fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
    let mut hashmap = HashMap::<i32, i32>::new();
    let mut min = i32::MAX;
    for vec in &mat {
        for num in vec {
            let val = hashmap.entry(*num).or_default();
            *val += 1;

            if *val == mat.len() as i32 {
                min = std::cmp::min(min, *num);
            }
        }
    }

    if min == i32::MAX {
        return -1;
    }

    min
}

pub fn main() {
    let mat = vec![
        vec![1, 2, 3, 4, 5],
        vec![2, 4, 5, 8, 10],
        vec![3, 5, 7, 9, 11],
        vec![1, 3, 5, 7, 9],
    ];
    println!("{}", smallest_common_element(mat));
}
