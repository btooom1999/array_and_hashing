use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { 
        return false;
    }

    let mut s_chars: HashMap<char, i32> = HashMap::new();
    let mut t_chars: HashMap<char, i32> = HashMap::new();

    for v in s.chars() {
        s_chars.entry(v).and_modify(|v| *v += 1).or_insert(1);
    }

    for v in t.chars() {
        t_chars.entry(v).and_modify(|v| *v += 1).or_insert(1);
    }

    s_chars == t_chars
}

pub fn main() {
    let s = "anagram".to_string();
    let t = "nagaram".to_string();
    
    println!("is_anagram: {}", is_anagram(s, t));
}
