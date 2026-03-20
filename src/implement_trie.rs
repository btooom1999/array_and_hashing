use std::collections::HashSet;

struct Trie {
    hashset: HashSet<String>,
    prefix: HashSet<String>
}

impl Trie {

    fn new() -> Self {
        Self {
            hashset: HashSet::default(),
            prefix: HashSet::default(),
        }
    }

    fn insert(&mut self, word: String) {
        for i in 1..=word.len() {
            self.prefix.insert(word[..i].to_string());
        }

        self.hashset.insert(word);
    }

    fn search(&self, word: String) -> bool {
        self.hashset.contains(&word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.prefix.contains(&prefix)
    }
}

pub fn main() {

}
