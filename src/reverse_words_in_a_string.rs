fn reverse_words(s: String) -> String {
    let mut result = String::new();
    let mut word = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            word.push(c);
        } else if !word.is_empty() {
            result = format!("{}{}{}", word, if result.is_empty() { "" } else { " " }, result);
            word.clear();
        }
    }

    if !word.is_empty() {
        result = format!("{}{}{}", word, if result.is_empty() { "" } else { " " }, result);
    }
    result
}

pub fn main() {
    let s = "the sky is blue".to_string();
    println!("{}", reverse_words(s));
}
