use std::collections::{HashMap, HashSet};

struct MagicDictionary {
    hashmap: HashMap<usize, Vec<Vec<u8>>>,
}

impl MagicDictionary {

    fn new() -> Self {
        Self { hashmap: HashMap::default() }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for dict in &dictionary {
            self.hashmap.entry(dict.len()).or_default().push(dict.clone().into_bytes());
        }
    }

    fn search(&self, search_word: String) -> bool {
        if let Some(list) = self.hashmap.get(&search_word.len()) {
            return list
                .iter()
                .filter(|&list| list != search_word.as_bytes())
                .any(|candidate| {
                    let mut once = false;

                    return candidate
                        .iter()
                        .zip(search_word.as_bytes())
                        .all(|(c1, c2)| {
                            if c1 != c2 {
                                if once { return false; }
                                once = true;
                            }

                            true
                        })
                })
        } else {
            return false;
        }
    }
}
