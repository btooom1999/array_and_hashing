use std::collections::HashMap;

fn max_difference(s: String) -> i32 {
    let mut hashmap = HashMap::new();

    for c in s.chars() {
        hashmap.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut max_odd = i32::MIN;
    let mut min_even = i32::MAX;

    for v in hashmap.into_values() {
        if v % 2 == 0 {
            min_even = std::cmp::min(min_even, v);
        } else {
            max_odd = std::cmp::max(max_odd, v);
        }
    }

    println!("{} {}", max_odd, min_even);

    max_odd - min_even
}

pub fn main() {
    let s = "mmsmsym".to_string();
    println!("{}", max_difference(s));
}
