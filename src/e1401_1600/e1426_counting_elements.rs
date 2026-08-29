use std::collections::HashSet;

fn count_elements(arr: Vec<i32>) -> i32 {
    let mut hashset = HashSet::<i32>::with_capacity(arr.len());
    for n in &arr {
        hashset.insert(*n);
    }

    let mut res = 0;
    for n in &arr {
        if hashset.contains(&(n + 1)) {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let arr = vec![1, 2, 3];
    println!("{}", count_elements(arr));
}
