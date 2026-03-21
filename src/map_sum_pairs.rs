use std::collections::HashMap;

struct MapSum {
    hashmap: HashMap<String, i32>,
    store: HashMap<String, i32>
}

impl MapSum {
    fn new() -> Self {
        Self { hashmap: HashMap::default(), store: HashMap::default() }
    }

    fn insert(&mut self, key: String, val: i32) {
        let old = self.store.get(&key);
        for i in 1..=key.len() {
            if let Some(&old) = old {
                self.hashmap.entry(key[..i].to_string()).and_modify(|v| *v = *v - old + val);
            } else {
                *self.hashmap.entry(key[..i].to_string()).or_default() += val;
            }
        }

        self.store.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        *self.hashmap.get(&prefix).unwrap_or(&0)
    }
}
