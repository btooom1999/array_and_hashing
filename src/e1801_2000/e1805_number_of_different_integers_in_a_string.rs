use std::collections::HashSet;

fn num_different_integers(mut word: String) -> i32 {
    word.push('a');
    let mut hashset = HashSet::new();

    let word = word.as_bytes();
    let n = word.len();
    let mut i = 0;
    for j in 0..n {
        if (j == 0 || word[j-1].is_ascii_lowercase()) && word[j].is_ascii_digit() {
            i = j;
        }
        if j+1 < n && word[j].is_ascii_digit() && word[j+1].is_ascii_lowercase() {
            while i < j && word[i] == b'0' {
                i += 1;
            }
            hashset.insert(word[i..=j].to_vec());
        }
    }

    hashset.len() as i32
}

pub fn main() {
    let word = "a123bc34d8ef34".to_string();
    println!("{}", num_different_integers(word));
}
