use std::collections::HashMap;

fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut hashmap = vec![0; 100_000];
    let is_vol = |b: u8| matches!(b, b'a' | b'e' | b'i' | b'u' | b'o');
    for (i, word) in words.iter().enumerate() {
        let bytes = word.as_bytes();
        if let (Some(first), Some(last)) = (bytes.first(), bytes.last()) {
            if is_vol(*first) && is_vol(*last) {
                hashmap[i] = 1;
            } else {
                hashmap[i] = 0;
            }
        }
    }

    queries
        .iter()
        .map(|q| hashmap[q[0] as usize..=q[1] as usize].iter().sum::<i32>())
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
