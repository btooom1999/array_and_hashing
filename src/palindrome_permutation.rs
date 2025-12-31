use std::collections::HashMap;

fn can_permute_palindrome(s: String) -> bool {
    let mut hashmap = HashMap::<char, i32>::with_capacity(s.len());
    for c in s.chars() {
        *hashmap.entry(c).or_default() += 1;
    }

    let mut count = 0;
    for v in hashmap.values() {
        if *v % 2 != 0 {
            count += 1;
        }

        if count >= 2 {
            return false;
        }
    }

    true
}

pub fn main() {
    let s = "aab".to_string();
    println!("{}", can_permute_palindrome(s));
}
