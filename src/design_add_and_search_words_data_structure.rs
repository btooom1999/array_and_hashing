use std::collections::HashMap;

struct WordDictionary {
    hashmap: HashMap<usize, Vec<String>>
}

impl WordDictionary {
    fn new() -> Self {
        Self { hashmap: HashMap::default() }
    }

    fn add_word(&mut self, word: String) {
        self.hashmap.entry(word.len()).or_default().push(word);
    }

    fn search(&self, word: String) -> bool {
        if let Some(dictionary) = self.hashmap.get(&word.len()) {
            return dictionary
                .iter()
                .any(|candidate| candidate
                    .chars()
                    .zip(word.chars())
                    .all(|(s1, s2)| s1 == s2 || s2 == '.')
                );
        }

        false
    }
}

pub fn main() {
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("bad".to_string());
    println!("{:?}", word_dictionary.hashmap);
}
