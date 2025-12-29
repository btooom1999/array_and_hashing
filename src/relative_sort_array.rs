use std::collections::HashMap;

fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::<i32, i32>::with_capacity(arr1.len());
    for num in &arr1 {
        *hashmap.entry(*num).or_default() += 1;
    }

    let mut res = Vec::new();
    for num in &arr2 {
        if let Some(n) = hashmap.get(num) {
            res.append(&mut vec![*num; *n as usize]);
            hashmap.remove(num);
        }
    }

    let mut exceed_res = Vec::new();
    for (k, v) in hashmap.into_iter() {
        exceed_res.append(&mut vec![k; v as usize]);
    }
    exceed_res.sort();
    res.append(&mut exceed_res);

    res
}

pub fn main() {
    let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr2 = vec![2, 1, 4, 3, 9, 6];
    println!("{:?}", relative_sort_array(arr1, arr2));
}
