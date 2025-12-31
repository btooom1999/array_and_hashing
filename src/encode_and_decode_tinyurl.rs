use std::collections::HashMap;

struct Codec {
    hashmap: HashMap<i32, String>,
    mapping: String,
}

impl Codec {
    fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
            mapping: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        let id = 459385;
        let mut n = id;
        let mut shortURL = Vec::new();
        while n > 0
            && let Some(c) = self.mapping.chars().nth(n as usize % 62)
        {
            shortURL.insert(0, c.to_string());
            n /= 62;
        }

        self.hashmap.insert(id, longURL);

        shortURL.join("")
    }

    fn convert_shortURL_to_ID(shortURL: String) -> i32 {
        let mut id = 0;

        // A simple base conversion logic
        for (i, c) in shortURL.chars().enumerate() {
            if c.is_ascii_lowercase() {
                id = id * 62 + (c as u8 - b'a') as i32;
            }
            if c.is_ascii_uppercase() {
                id = id * 62 + (c as u8 - b'A') as i32 + 26;
            }
            if c.is_ascii_digit() {
                id = id * 62 + (c as u8 - b'0') as i32 + 52;
            }
        }

        id
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        let id = Self::convert_shortURL_to_ID(shortURL);
        self.hashmap.get(&id).unwrap_or(&String::new()).to_owned()
    }
}

pub fn main() {
    let mut codec = Codec::new();
    let shortURL = codec.encode("https://leetcode.com/problems/design-tinyurl".to_string());
    println!("tinyURL: {}", shortURL);
    let longURL = codec.decode(shortURL.clone());
    println!("longURL: {}", codec.decode(shortURL));
}
