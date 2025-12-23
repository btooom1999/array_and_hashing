use std::collections::HashMap;

fn is_isomorphic(s: String, t: String) -> bool {
    let mut hashmap_t = HashMap::with_capacity(s.len());
    let mut hashmap_s = HashMap::with_capacity(t.len());
    for (cs, ct) in s.chars().zip(t.chars()) {
        hashmap_s.entry(cs).or_insert(ct);
        hashmap_t.entry(ct).or_insert(cs);

        if let Some(val) = hashmap_s.get(&cs)
            && *val != ct
        {
            return false;
        }
        if let Some(val) = hashmap_t.get(&ct)
            && *val != cs
        {
            return false;
        }
    }

    true
}

pub fn main() {
    let s = "egg".to_string();
    let t = "add".to_string();

    println!("{}", is_isomorphic(s, t));
}
