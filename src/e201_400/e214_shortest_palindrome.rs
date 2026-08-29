fn shortest_palindrome(s: String) -> String {
    if s.is_empty() { return s; }

    let reverse = s.chars().rev().collect::<String>();
    let pattern = format!("{}#{}", s, reverse).into_bytes();
    let n = pattern.len();
    let mut lps = vec![0; n];
    let mut i = 1;
    let mut len = 0;
    while i < n {
        if pattern[i] == pattern[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len == 0 {
            i += 1;
        } else {
            len = lps[len-1];
        }
    }

    let prefix = &reverse[..reverse.len() - lps[n-1]];
    format!("{prefix}{s}")
}

// abcd#dcba

pub fn main() {
    let s = "abcd".to_string();
    println!("{}", shortest_palindrome(s));
}
