use std::collections::HashSet;

fn partition_string(s: String) -> Vec<String> {
    let mut hashset = HashSet::new();
    let mut prev_str = String::new();
    let mut res = Vec::new();
    for c in s.chars() {
        prev_str.push(c);

        if !hashset.contains(&prev_str) {
            res.push(prev_str.clone());
            hashset.insert(prev_str.clone());
            prev_str.clear();
        }
    }

    res
}

pub fn main() {
    let s = "abbccccd".to_string();
    println!("{:?}", partition_string(s));
}
