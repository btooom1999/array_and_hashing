use std::collections::HashMap;

fn max_length_between_equal_characters(s: String) -> i32 {
    let mut hashmap = HashMap::<char, i32>::with_capacity(s.len());
    let mut max = -1;
    for (i, c) in s.chars().enumerate() {
        if let Some(val) = hashmap.get(&c) {
            max = std::cmp::max(max, i as i32 - val - 1);
        } else {
            hashmap.insert(c, i as i32);
        }
    }

    max
}

pub fn main() {
    let s = "aa".to_string();
    println!("{}", max_length_between_equal_characters(s));
}
