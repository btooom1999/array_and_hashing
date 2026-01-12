use std::collections::HashMap;

fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut hashmap = HashMap::<i32, i32>::with_capacity(arr.len());
    let mut max = 0;
    for num in arr {
        let val = hashmap.entry(num).or_default();
        *val += 1;
        max = std::cmp::max(max, *val);
    }

    let mut hashset = vec![false; max as usize + 1];
    for num in hashmap.into_values() {
        if !hashset[num as usize] {
            hashset[num as usize] = true;
        } else {
            return false;
        }
    }

    true
}

pub fn main() {
    let arr = vec![1,2,2,1,1,3];
    println!("{}", unique_occurrences(arr));
}
