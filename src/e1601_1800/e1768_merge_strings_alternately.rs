fn merge_alternately(word1: String, word2: String) -> String {
    let word1 = word1.chars().collect::<Vec<_>>();
    let word2 = word2.chars().collect::<Vec<_>>();
    let mut str = String::new();
    for i in 0..std::cmp::max(word1.len(), word2.len()) {
        if let Some(val) = word1.get(i) {
            str.push(*val);
        }
        if let Some(val) = word2.get(i) {
            str.push(*val);
        }
    }

    str
}

pub fn main() {
    let word1 = "abc".to_string();
    let word2 = "pqr".to_string();
    println!("{}", merge_alternately(word1, word2));
}
