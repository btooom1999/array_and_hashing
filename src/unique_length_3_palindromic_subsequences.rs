use std::collections::HashSet;

fn count_palindromic_subsequence(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut first = [n; 26];
    let mut last = [0; 26];

    for (i, c) in s.iter().enumerate() {
        let k = (*c as u8 - b'a') as usize;
        if first[k] == n {
            first[k] = i;
        }
        last[k] = i;
    }

    let mut res = 0;
    for k in 0..26 {
        if first[k] < last[k] {
            let mut chars = [false; 26];
            for i in first[k] + 1..last[k] {
                chars[(s[i] as u8 - b'a') as usize] = true;
            }
            res += chars.iter().filter(|v| **v).count();
        }
    }

    res as i32
}

pub fn main() {
    let s = "abbccbabac".to_string();
    println!("{}", count_palindromic_subsequence(s));
}
