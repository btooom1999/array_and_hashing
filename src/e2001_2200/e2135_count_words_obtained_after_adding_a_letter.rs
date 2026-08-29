use std::collections::HashSet;

fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
    let mut hashset = HashSet::new();
    for word in start_words {
        let mut bit = 0;
        for b in word.as_bytes() {
            bit ^= 1 << (b-b'a');
        }
        hashset.insert(bit);
    }

    let mut res = 0;
    for word in target_words {
        let mut bit = 0;
        for b in word.as_bytes() {
            bit ^= 1 << (b-b'a');
        }
        for i in 0..26 {
            if (bit >> i) & 1 == 1 && hashset.contains(&(bit ^ (1 << i))) {
                res += 1;
                break;
            }
        }
    }

    res
}

pub fn main() {
    let start_words = ["g","vf","ylpuk","nyf","gdj","j","fyqzg","sizec"].into_iter().map(String::from).collect();
    let target_words = ["r","am","jg","umhjo","fov","lujy","b","uz","y"].into_iter().map(String::from).collect();
    println!("{}", word_count(start_words, target_words));
}
