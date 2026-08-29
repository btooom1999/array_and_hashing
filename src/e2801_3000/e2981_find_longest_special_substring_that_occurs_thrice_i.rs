use std::collections::HashMap;

fn maximum_length(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut hashmap = HashMap::<_, i32>::new();
    for i in (0..n).rev() {
        for j in i..n {
            if j > i && s[j] != s[j-1] {
                break;
            }
            *hashmap.entry(s[i..j+1].to_vec()).or_default() += 1;
        }
    }

    let mut longest = -1;
    for (k, v) in hashmap {
        if v >= 3 {
            longest = longest.max(k.len() as i32);
        }
    }
    longest
}

pub fn main() {
    let s = "abcdabcddddabcddddccccbbbbaaaa".to_string();
    println!("{}", maximum_length(s));
}
