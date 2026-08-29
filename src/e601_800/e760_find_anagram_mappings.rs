use std::collections::HashMap;

fn anagram_mappings(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut hashmap = HashMap::<i32, Vec<i32>>::with_capacity(b.len());
    for (i, num) in b.into_iter().enumerate() {
        hashmap.entry(num).or_default().push(i as i32);
    }

    let mut res = Vec::new();
    for num in a {
        let vec = hashmap.entry(num).or_default();
        if let Some(val) = vec.first() {
            res.push(*val);
            vec.remove(0);
        } else {
            res.push(-1);
        }
    }

    res
}

pub fn main() {
    let a = vec![12, 28, 46, 32, 50];
    let b = vec![50, 12, 32, 46, 28];
    println!("{:?}", anagram_mappings(a, b));
}
