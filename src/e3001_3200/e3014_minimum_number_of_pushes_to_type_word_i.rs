use std::collections::HashMap;

fn minimum_pushes(word: String) -> i32 {
    let mut hashmap = HashMap::new();
    let mut res = 0;
    for c in word.chars() {
        if let Some(&value) = hashmap.get(&c) {
            res += value;
        } else {
            let value = ((hashmap.len()+1) as f32 / 8f32).ceil() as i32;
            hashmap.insert(c, value);
            res += value;
        }
    }

    res
}

pub fn main() {
    let word = "abhrlngxyjkezwcm".to_string();
    println!("{}", minimum_pushes(word));
}
