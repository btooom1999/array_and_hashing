use std::collections::HashSet;

fn minimized_string_length(s: String) -> i32 {
    let mut hashset = HashSet::new();
    for byte in s.as_bytes() {
        hashset.insert(*byte);
    }

    hashset.len() as i32
}

pub fn main() {
    let s = "baadccab".to_string();
    println!("{}", minimized_string_length(s));
}
