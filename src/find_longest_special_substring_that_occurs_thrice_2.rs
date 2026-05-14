use std::collections::HashMap;

fn maximum_length(s: String) -> i32 {
    let s = s.as_bytes();
    let mut chars = [0;26];
    let mut hashmap = HashMap::<_, i32>::new();
    let mut i = 0;
    for j in 0..s.len() {
        let k = (s[j] - b'a') as usize;
        if s[j] != s[i] {
            i = j;
        }

        let range = chars[k]+1..=(j-i+1) as i32;
        for i in range.rev() {
            let val = hashmap.entry((s[j], i)).or_default();
            *val += 1;
            if *val == 3 {
                chars[k] = i;
                hashmap.remove(&(s[j], i));
            }
        }
    }

    let val = chars.into_iter().max().unwrap();
    if val == 0 { -1 } else { val }
}

pub fn main() {
    // let s = "abcdef".to_string();
    let s = vec!["a".to_string(); 500_000].join("");
    println!("{}", maximum_length(s));
}
