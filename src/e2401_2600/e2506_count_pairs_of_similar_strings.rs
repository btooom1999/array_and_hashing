use std::collections::HashMap;

fn similar_pairs(words: Vec<String>) -> i32 {
    let mut hashmap = HashMap::<u32, i32>::new();
    let mut res = 0;
    for word in words {
        let mut mask = 0u32;
        for &b in word.as_bytes() {
            let pos = (b - b'a') as u32;
            if mask & (1 << pos) == 0 {
                mask |= 1 << pos;
            }
        }

        hashmap.entry(mask).and_modify(|v| {
            res += *v;
            *v += 1;
        }).or_insert(1);
    }

    res
}

pub fn main() {
    let words = ["aba","aabb","abcd","bac","aabc"].into_iter().map(String::from).collect();
    println!("{}", similar_pairs(words));
}
