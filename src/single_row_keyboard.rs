use std::collections::HashMap;

fn calculate_time(keyboard: String, word: String) -> i32 {
    let mut hashmap = HashMap::<char, i32>::with_capacity(keyboard.len());
    for (i, v) in keyboard.chars().enumerate() {
        hashmap.insert(v, i as i32);
    }

    let mut sum = 0;
    let mut time = None;
    for c in word.chars() {
        if let Some(val) = time {
            if let (Some(val1), Some(val2)) = (hashmap.get(&c), hashmap.get(&val)) {
                sum += (val1 - val2).abs();
            }
        } else {
            sum += hashmap.get(&c).unwrap();
        }

        time = Some(c);
    }

    sum
}

pub fn main() {
    let keyboard = "pqrstuvwxyzabcdefghijklmno".to_string();
    let word = "leetcode".to_string();
    println!("{}", calculate_time(keyboard, word));
}
