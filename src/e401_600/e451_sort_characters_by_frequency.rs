use std::collections::{BinaryHeap, HashMap};

fn frequency_sort(s: String) -> String {
    let mut hashmap = HashMap::new();
    for c in s.chars() {
        *hashmap.entry(c).or_insert(0) += 1;
    }

    let mut str = String::new();
    let mut max_heap = hashmap.into_iter().map(|v| (v.1, v.0)).collect::<BinaryHeap<_>>();
    while let Some(data) = max_heap.pop() {
        str.push_str(&vec![data.1; data.0].iter().collect::<String>());
    }

    str
}

pub fn main() {
    let s = "tree".to_string();
    println!("{}", frequency_sort(s));
}
