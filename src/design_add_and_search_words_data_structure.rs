use std::collections::HashSet;

struct WordDictionary {
    hashset: HashSet<String>
}

impl WordDictionary {
    fn new() -> Self {
        Self { hashset: HashSet::default() }
    }

    fn add_word(&mut self, word: String) {
        self.hashset.insert(word);
    }

    fn search(&self, word: String) -> bool {
        let mut word = word.into_bytes();
        let (mut first_dot, mut second_dot) = (-1, -1);
        for i in 0..word.len() as i32 {
            let char = word[i as usize];
            if char == b'.' {
                if first_dot != -1 {
                    second_dot = i;
                } else {
                    first_dot = i;
                }
            }
        }

        if first_dot != -1 && second_dot != -1 {
            for i in 0..26 {
                for j in 0..26 {
                    word[first_dot as usize] = b'a' + i;
                    word[second_dot as usize] = b'a' + j;

                    if self.hashset.contains(&String::from_utf8(word.clone()).unwrap()) {
                        return true;
                    }
                }
            }

            return false;
        }

        if first_dot != -1 {
            for i in 0..26 {
                word[first_dot as usize] = b'a' + i;

                if self.hashset.contains(&String::from_utf8(word.clone()).unwrap()) {
                    return true;
                }
            }

            return false;
        }

        self.hashset.contains(&String::from_utf8(word).unwrap())
    }
}

pub fn main() {
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("bad".to_string());
    println!("{:?}", word_dictionary.hashset);
}
