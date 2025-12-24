use std::collections::HashMap;

fn word_pattern(pattern: String, s: String) -> bool {
    let pattern = pattern.as_bytes();
    let s = s.split(' ').collect::<Vec<&str>>();

    if pattern.len() != s.len() {
        return false;
    }

    let mut hashmap = HashMap::<i32, &str>::with_capacity(pattern.len());

    for i in 0..s.len() {
        let k = (pattern[i] - b'a') as i32;
        if let Some(val) = hashmap.get(&k) {
            if *val != s[i] {
                return false;
            }
        } else {
            if hashmap.values().any(|v| *v == s[i]) {
                return false;
            }
            hashmap.insert(k, s[i]);
        }
    }

    true
}

pub fn main() {
    let pattern = "abba".to_string();
    let s = "dog dog dog dog".to_string();
    println!("{}", word_pattern(pattern, s));
}
