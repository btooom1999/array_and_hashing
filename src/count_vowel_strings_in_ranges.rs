use std::collections::HashMap;

fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut hashmap = vec![(0, false); 100_000];
    let is_vol = |b: u8| matches!(b, b'a' | b'e' | b'i' | b'u' | b'o');
    for (i, word) in words.iter().enumerate() {
        let bytes = word.as_bytes();
        if let (Some(first), Some(last)) = (bytes.first(), bytes.last()) {
            let prev_val = if i == 0 { 0 } else { hashmap[i - 1].0 };
            if is_vol(*first) && is_vol(*last) {
                hashmap[i] = (1 + prev_val, true);
            } else {
                hashmap[i] = (prev_val, false);
            }
        }
    }

    queries
        .iter()
        .map(|q| {
            let val = hashmap[q[1] as usize].0 - hashmap[q[0] as usize].0;
            if hashmap[q[0] as usize].1 {
                return val + 1;
            }
            val
        })
        .collect::<_>()
}

pub fn main() {
    let words = vec![
        "aba".to_string(),
        "bcb".to_string(),
        "ece".to_string(),
        "aa".to_string(),
        "e".to_string(),
    ];
    let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
    println!("{:?}", vowel_strings(words, queries));
}
