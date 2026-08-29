use std::collections::HashMap;

fn longest_palindrome(s: String) -> i32 {
    let mut hashmap = HashMap::<char, i32>::with_capacity(s.len());
    for c in s.chars() {
        *hashmap.entry(c).or_default() += 1;
    }

    let mut flag = true;
    let mut sum = 0;
    for val in hashmap.values() {
        if flag {
            sum += val;
        } else {
            sum += val - (val % 2)
        }

        if val % 2 != 0 {
            flag = false;
        }
    }

    sum
}

pub fn main() {
    let s = "abccccdd".to_string();
    println!("{}", longest_palindrome(s));
}
