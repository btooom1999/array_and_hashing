use std::collections::HashMap;

fn partition_labels(s: String) -> Vec<i32> {
    let s = s.into_bytes();
    let mut hashmap = HashMap::<u8, i32>::new();
    for &b in &s {
        *hashmap.entry(b).or_default() += 1;
    }

    let mut res = Vec::new();
    let mut i = 0;
    let mut remaining = HashMap::<u8, i32>::new();
    for (j, b) in s.into_iter().enumerate() {
        if hashmap.contains_key(&b) {
            remaining.insert(b, hashmap.remove(&b).unwrap());
        }

        let val = remaining.get_mut(&b).unwrap();
        *val -= 1;
        if *val == 0 {
            remaining.remove(&b);
        }

        if remaining.is_empty() {
            res.push((j-i+1) as i32);
            i = j+1;
        }
    }

    res
}

pub fn main() {
    let s = "ababcbacadefegdehijhklij".to_string();
    println!("{:?}", partition_labels(s));
}

