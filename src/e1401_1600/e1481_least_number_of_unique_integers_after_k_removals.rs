use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
    let mut hashmap = HashMap::new();
    for num in arr {
        *hashmap.entry(num).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::from(hashmap.into_values().map(Reverse).collect::<Vec<_>>());
    while k > 0 {
        if let Some(Reverse(data)) = heap.pop() && data > 1 {
            heap.push(Reverse(data-1));
        }
        k -= 1;
    }

    heap.len() as _
}

pub fn main() {
    let arr = [5,5,4];
    let k = 1;
    println!("{}", find_least_num_of_unique_ints(arr.into(), k));
}
