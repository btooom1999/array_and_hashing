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
        let delta = val - self.store.get(&key).unwrap_or(&0);
        for i in 1..=key.len() {
            *self.hashmap.entry(key[..i].to_string()).or_default() += delta;
        }

        self.store.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        *self.hashmap.get(&prefix).unwrap_or(&0)
    }
}

pub fn main() {

}
