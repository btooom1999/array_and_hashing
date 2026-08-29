use std::collections::HashMap;

fn min_deletions(s: String) -> i32 {
    let mut hashmap = HashMap::new();

    for c in s.chars() {
        hashmap.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut vec = vec![0; 100_000];
    for (_, count) in hashmap {
        vec[count as usize] += 1;
    }

    let mut res = 0;
    let mut stack = Vec::new();
    for (len, mut count) in vec.into_iter().enumerate().skip(1) {
        if count == 0 {
            stack.push(len);
        } else if count > 1 {
            while count != 1 {
                res += len - stack.pop().unwrap_or(0);
                count -= 1;
            }
        }
    }

    res as i32
}

pub fn main() {
    let s = "ceabaacb".to_string();
    println!("{:?}", min_deletions(s));
}
