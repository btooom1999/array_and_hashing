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
    fn encode(&mut self, long_url: String) -> String {
        let id = 459385;
        let mut n = id;
        let mut short_url = Vec::new();
        while n > 0
            && let Some(c) = self.mapping.chars().nth(n as usize % 62)
        {
            short_url.insert(0, c.to_string());
            n /= 62;
        }

        self.hashmap.insert(id, long_url);

        short_url.join("")
    }

    fn convert_short_url_to_id(short_url: String) -> i32 {
        let mut id = 0;

        // A simple base conversion logic
        for c in short_url.chars() {
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
    fn decode(&self, short_url: String) -> String {
        let id = Self::convert_short_url_to_id(short_url);
        self.hashmap.get(&id).unwrap_or(&String::new()).to_owned()
    }
}

pub fn main() {
    let mut codec = Codec::new();
    let short_url = codec.encode("https://leetcode.com/problems/design-tinyurl".to_string());
    println!("tinyURL: {}", short_url);
    // let long_url = codec.decode(short_url.clone());
    println!("longURL: {}", codec.decode(short_url));
}
