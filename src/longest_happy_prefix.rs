fn longest_prefix(s: String) -> String {
    let n = s.len();
    let s = s.as_bytes();
    for i in (0..n-1).rev() {
        let prefix = &s[..=i];
        let suffix = &s[n-i-1..];
        if prefix == suffix {
            return String::from_utf8(prefix.to_vec()).unwrap();
        }
    }

    String::new()
}

pub fn main() {
    let s = "level".to_string();
    println!("{}", longest_prefix(s));
}
