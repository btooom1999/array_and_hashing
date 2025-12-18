use std::collections::HashMap;

pub fn main() {
    let s = "anagram";
    let t = "nagaram";
    let mut is_anagram = true;

    let mut chars: HashMap<char, i32> = HashMap::new();

    for char in s.chars() {
        if let Some(value) = chars.get_mut(&char) {
            *value += 1;
        } else {
            chars.insert(char, 1);
        }
    }

    for char in t.chars() {
        if let Some(value) = chars.get_mut(&char) {
            *value -= 1;
        }
    }

    for (_, value) in chars.iter() {
        if *value > 0 {
            is_anagram = false;
            break;
        }
    }

    println!("is_anagram: {}", is_anagram);
}
