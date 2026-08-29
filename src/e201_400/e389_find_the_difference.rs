fn find_the_difference(mut s: String, mut t: String) -> char {
    let mut res = 0;
    while !s.is_empty() || !t.is_empty() {
        let a = s.pop().map_or(0, |v| v as u8);
        let b = t.pop().map_or(0, |v| v as u8);
        res ^= a ^ b;
    }

    res as char
}

pub fn main() {
    let s = "abcd".to_string();
    let t = "abcde".to_string();
    println!("{}", find_the_difference(s, t));
}
